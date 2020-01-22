mod help;

use cassandra_cpp::*;
use std::time::SystemTime;
use time::Duration;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
struct Udt {
    dt: u32,
    tm: i64,
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
struct Basic {
    bln: bool,
    flt: f32,
    dbl: f64,
    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    ts: i64,
    addr: Inet,
    tu: Uuid,
    id: Uuid,
    ct: Udt,
}

/// Create the table for basic testing.
fn create_basic_table(session: &Session) {
    let type_statement = &stmt!("CREATE TYPE IF NOT EXISTS examples.udt (dt date, tm time);");
    session.execute(type_statement).wait().unwrap();
    let table_statement = &stmt!(
        "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt \
         float, dbl double, i8 tinyint, i16 smallint, i32 int, i64 bigint, \
         ts timestamp, addr inet, tu timeuuid, id uuid, ct udt, PRIMARY KEY (key));"
    );
    session.execute(table_statement).wait().unwrap();
    let truncate_statement = &stmt!("TRUNCATE examples.basic;");
    session.execute(truncate_statement).wait().unwrap();
}

fn insert_into_basic(session: &Session, key: &str, basic: &Basic) -> Result<CassResult> {
    let mut statement = stmt!(
        "INSERT INTO examples.basic (key, bln, flt, dbl, i8, i16, i32, i64, ts, addr, tu, id, ct) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);"
    );

    let ct_type = DataType::new_udt(2);
    ct_type.add_sub_value_type_by_name::<&str>("dt", ValueType::DATE)?;
    ct_type.add_sub_value_type_by_name::<&str>("tm", ValueType::TIME)?;
    let mut ct_udt = ct_type.new_user_type();
    ct_udt.set_uint32_by_name("dt", basic.ct.dt)?;
    ct_udt.set_int64_by_name("tm", basic.ct.tm)?;

    statement.bind(0, key)?;
    statement.bind(1, basic.bln)?;
    statement.bind(2, basic.flt)?;
    statement.bind(3, basic.dbl)?;
    statement.bind(4, basic.i8)?;
    statement.bind(5, basic.i16)?;
    statement.bind(6, basic.i32)?;
    statement.bind(7, basic.i64)?;
    statement.bind(8, basic.ts)?;
    statement.bind(9, basic.addr)?;
    statement.bind(10, basic.tu)?;
    statement.bind(11, basic.id)?;
    statement.bind(12, &ct_udt)?;

    session.execute(&statement).wait()
}

const SELECT_QUERY: &str = "SELECT key, bln, flt, dbl, i8, i16, i32, i64, ts, addr, tu, id, ct \
                            FROM examples.basic WHERE key = ?";

fn select_from_basic(session: &Session, key: &str) -> Result<Option<Basic>> {
    let mut statement = stmt!(SELECT_QUERY);
    statement.bind_string(0, key)?;
    let result = session.execute(&statement).wait()?;
    println!("Result: \n{:?}\n", result);
    match result.first_row() {
        None => Ok(None),
        Some(row) => {
            let fields_iter: UserTypeIterator = row.get(12)?;
            let mut dt: u32 = 0;
            let mut tm: i64 = 0;
            for field in fields_iter {
                match field.0.as_ref() {
                    "dt" => dt = field.1.get_u32()?,
                    "tm" => tm = field.1.get_i64()?,
                    _ => panic!("Unexpected field: {:?}", field),
                }
            }

            Ok(Some(Basic {
                bln: row.get(1)?,
                flt: row.get(2)?,
                dbl: row.get(3)?,
                i8: row.get(4)?,
                i16: row.get(5)?,
                i32: row.get(6)?,
                i64: row.get(7)?,
                ts: row.get(8)?,
                addr: row.get(9)?,
                tu: row.get(10)?,
                id: row.get(11)?,
                ct: Udt { dt: dt, tm: tm },
            }))
        }
    }
}

fn select_from_basic_prepared(
    session: &Session,
    prepared: &PreparedStatement,
    key: &str,
    basic: &mut Basic,
) -> Result<()> {
    let mut statement = prepared.bind();
    statement.bind_string(0, key)?;
    let future = session.execute(&statement);
    let result = future.wait()?;
    println!("{:?}", result);
    for row in result.iter() {
        basic.bln = row.get(1)?;
        basic.flt = row.get(2)?;
        basic.dbl = row.get(3)?;
        basic.i8 = row.get(4)?;
        basic.i16 = row.get(5)?;
        basic.i32 = row.get(6)?;
        basic.i64 = row.get(7)?;
        basic.ts = row.get(8)?;
        basic.addr = row.get(9)?;
        basic.tu = row.get(10)?;
        basic.id = row.get(11)?;

        let fields_iter: UserTypeIterator = row.get(12)?;
        let mut dt: u32 = 0;
        let mut tm: i64 = 0;

        for field in fields_iter {
            match field.0.as_ref() {
                "dt" => dt = field.1.get_u32()?,
                "tm" => tm = field.1.get_i64()?,
                _ => panic!("Unexpected field: {:?}", field),
            }
        }

        basic.ct = Udt { dt: dt, tm: tm };
    }
    Ok(())
}

#[test]
fn test_simple() {
    let query = stmt!("SELECT keyspace_name FROM system_schema.keyspaces;");
    let col_name = "keyspace_name";

    let session = help::create_test_session();

    let result = session.execute(&query).wait().unwrap();
    println!("{}", result);
    let mut names = vec![];
    for row in result.iter() {
        let col: String = row.get_by_name(col_name).unwrap();
        println!("ks name = {}", col);
        names.push(col);
    }

    assert!(names.contains(&"system_schema".to_string()));
    assert!(names.contains(&"system_auth".to_string()));
}

#[test]
fn test_basic_error() {
    let session = help::create_test_session();
    let s = stmt!("CREATE GOBBLEDEGOOK;");
    session
        .execute(&s)
        .wait()
        .expect_err("Should cleanly return an error");
}

#[test]
fn test_basic_round_trip() {
    let session = help::create_test_session();
    help::create_example_keyspace(&session);
    create_basic_table(&session);
    let uuid_gen = UuidGen::default();

    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        * 1_000;
    let input = Basic {
        bln: true,
        flt: 0.001f32,
        dbl: 0.0002f64,
        i8: 1,
        i16: 2,
        i32: 3,
        i64: 4,
        ts: ts as i64,
        addr: "127.0.0.1".parse().unwrap(),
        tu: uuid_gen.gen_time(),
        id: uuid_gen.gen_random(),
        ct: Udt {
            dt: ts as u32,
            tm: ts as i64,
        },
    };

    insert_into_basic(&session, "test", &input).unwrap();
    let output = select_from_basic(&session, "test")
        .unwrap()
        .expect("no output from select");

    println!("{:?}", input);
    println!("{:?}", output);

    assert!(input == output);
}

#[test]
fn test_prepared_round_trip() {
    let session = help::create_test_session();
    help::create_example_keyspace(&session);
    create_basic_table(&session);
    let uuid_gen = UuidGen::default();

    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        * 1_000;
    let input = Basic {
        bln: true,
        flt: 0.001f32,
        dbl: 0.0002f64,
        i8: 1,
        i16: 2,
        i32: 3,
        i64: 4,
        ts: ts as i64,
        addr: "127.0.0.1".parse().unwrap(),
        tu: uuid_gen.gen_time(),
        id: uuid_gen.gen_random(),
        ct: Udt {
            dt: ts as u32,
            tm: ts as i64,
        },
    };
    let mut output = Basic::default();

    println!("Basic insertions");
    insert_into_basic(&session, "prepared_test", &input).unwrap();
    println!("Preparing");
    let prepared = session
        .prepare(SELECT_QUERY)
        .unwrap()
        .wait()
        .expect("prepared");
    select_from_basic_prepared(&session, &prepared, "prepared_test", &mut output).unwrap();
    assert_eq!(input, output, "Input:  {:?}\noutput: {:?}", &input, &output);
}

#[test]
fn test_null_retrieval() {
    let session = help::create_test_session();
    help::create_example_keyspace(&session);
    create_basic_table(&session);

    // Insert a partial row.
    let partial =
        stmt!("INSERT INTO examples.basic (key, bln, flt) VALUES ('vacant', true, 3.14);");
    session.execute(&partial).wait().expect("insert");

    // Read the whole row.
    let query = stmt!(
        "SELECT key, bln, flt, dbl, i8, i16, i32, i64, ts, addr, tu, id, ct \
         FROM examples.basic WHERE key = 'vacant';"
    );
    let result = session.execute(&query).wait().expect("select");

    // Check response is as expected.
    assert_eq!(1, result.row_count());
    let row = result.first_row().unwrap();

    let v: String = row.get(0).unwrap();
    assert_eq!(v, "vacant".to_string());
    assert!(!row.get_column(0).unwrap().is_null());

    let v: bool = row.get(1).unwrap();
    assert_eq!(v, true);
    assert!(!row.get_column(1).unwrap().is_null());

    let v: f32 = row.get(2).unwrap();
    assert_eq!(v, 3.14f32);
    assert!(!row.get_column(2).unwrap().is_null());

    let c = row.get_column(3).expect("should be present");
    c.get_f64().expect_err("should be null");
    assert!(c.is_null());

    let c = row.get_column(4).expect("should be present");
    c.get_i8().expect_err("should be null");
    assert!(c.is_null());

    let c = row.get_column(5).expect("should be present");
    c.get_i16().expect_err("should be null");
    assert!(c.is_null());

    let c = row.get_column(6).expect("should be present");
    c.get_i32().expect_err("should be null");
    assert!(c.is_null());

    let c = row.get_column(7).expect("should be present");
    c.get_i64().expect_err("should be null");
    assert!(c.is_null());

    let c = row.get_column(8).expect("should be present");
    c.get_u32().expect_err("should be null");
    assert!(c.is_null());

    let c = row.get_column(9).expect("should be present");
    c.get_inet().expect_err("should be null");
    assert!(c.is_null());

    let c = row.get_column(10).expect("should be present");
    c.get_uuid().expect_err("should be null");
    assert!(c.is_null());

    let c = row.get_column(11).expect("should be present");
    c.get_uuid().expect_err("should be null");
    assert!(c.is_null());

    let c = row.get_column(12).expect("should be present");
    c.get_user_type().expect_err("should be null");
    assert!(c.is_null());
}

#[test]
fn test_null_insertion() {
    let session = help::create_test_session();
    help::create_example_keyspace(&session);
    create_basic_table(&session);

    // Insert some explicit nulls.
    let mut s = stmt!(
        "INSERT INTO examples.basic (key, bln, flt, dbl, i8, i16, i32, i64, ts, addr, tu, id, ct) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);"
    );
    s.bind(0, "shrdlu").unwrap();
    s.bind(1, false).unwrap();
    s.bind_null(2).unwrap();
    s.bind(3, 2.72f64).unwrap();
    // deliberately omit 4 - this should be equivalent to binding null
    s.bind_null(5).unwrap();
    s.bind_null(6).unwrap();
    s.bind_null(7).unwrap();
    s.bind_null(8).unwrap();
    s.bind_null(9).unwrap();
    s.bind_null(10).unwrap();
    s.bind_null(11).unwrap();
    s.bind_null(12).unwrap();
    session.execute(&s).wait().unwrap();

    // Read them back.
    let s = stmt!(
        "SELECT key, bln, flt, dbl, i8, i16, i32, i64, ts, addr, tu, id, ct \
         FROM examples.basic WHERE key = 'shrdlu';"
    );
    let result = session.execute(&s).wait().expect("select");
    assert_eq!(1, result.row_count());
    let row = result.first_row().unwrap();

    assert!(!row.get_column(0).unwrap().is_null());
    assert!(!row.get_column(1).unwrap().is_null());
    assert!(row.get_column(2).unwrap().is_null());
    assert!(!row.get_column(3).unwrap().is_null());
    assert!(row.get_column(4).unwrap().is_null());
    assert!(row.get_column(5).unwrap().is_null());
    assert!(row.get_column(6).unwrap().is_null());
    assert!(row.get_column(7).unwrap().is_null());
    assert!(row.get_column(8).unwrap().is_null());
    assert!(row.get_column(9).unwrap().is_null());
    assert!(row.get_column(10).unwrap().is_null());
    assert!(row.get_column(11).unwrap().is_null());
    assert!(row.get_column(12).unwrap().is_null());
}

/// Check for a needle in a haystack, and fail if not present.
fn assert_contains(haystack: String, needle: &str) {
    assert!(
        haystack.contains(needle),
        "assert_contains: `{}` not found in `{}`",
        needle,
        &haystack
    );
}

#[test]
fn test_rendering() {
    let query = stmt!("SELECT * FROM system_schema.tables;");
    let session = help::create_test_session();
    let result = session.execute(&query).wait().unwrap();
    println!("test_rendering: {}", result);

    let row = result.iter().next().unwrap();
    let compaction_col = row.get_column_by_name("compaction").unwrap();

    // Check rendering of a column
    let column_debug = format!("{:?}", compaction_col);
    println!("Column debug: {}", &column_debug);
    assert_contains(
        column_debug,
        r#"{"class" => "org.apache.cassandra.db.compaction"#,
    );

    let column_display = format!("{}", compaction_col);
    println!("Column display: {}", &column_display);
    assert_contains(
        column_display,
        r#"{class => org.apache.cassandra.db.compaction"#,
    );

    // Check retrieving a string and a str.
    let keyspace_col = row.get_column_by_name("keyspace_name").unwrap();
    let str: &str = keyspace_col.get_str().unwrap();
    println!("Str is {}", str);
    assert!(str.len() > 0, "empty str");
    let str: String = keyspace_col.get_string().unwrap();
    println!("String is {}", str);
    assert!(str.len() > 0, "empty string");

    // Check invalid retrieval type.
    keyspace_col
        .get_map()
        .expect_err("Should fail with invalid value type");
}

#[test]
fn test_error_reporting() {
    let session = help::create_test_session();

    // Simple error.
    let mut query = stmt!("SELECT * from system_schema.tables;");
    query.set_consistency(Consistency::THREE).unwrap(); // assuming we only have one node, this must fail
    let err = session
        .execute(&query)
        .wait()
        .expect_err("Should have failed!");
    println!("Got error {} kind {:?}", err, err.kind());
    match *err.kind() {
        ErrorKind::CassError(CassErrorCode::LIB_NO_HOSTS_AVAILABLE, _) => (),
        ref k => panic!("Unexpected error kind {}", k),
    }

    // Detailed error.
    let query = stmt!("SELECT gibberish from system_schema.tables;");
    let err = session
        .execute(&query)
        .wait()
        .expect_err("Should have failed!");
    println!("Got error {} kind {:?}", err, err.kind());
    match *err.kind() {
        ErrorKind::CassErrorResult(
            CassErrorCode::SERVER_INVALID_QUERY,
            _,
            _,
            -1,
            -1,
            -1,
            _,
            _,
            _,
            _,
            _,
        ) => (),
        ref k => panic!("Unexpected error kind {}", k),
    }

    // UTF-8 error - return an invalid UTF-8 string
    // Interpret -1 (0xFFFFFFFF) as a UTF-8 string, but 0xFF... is invalid UTF-8.
    help::create_example_keyspace(&session);
    create_basic_table(&session);
    let s = stmt!("INSERT INTO examples.basic (key, i32) VALUES ('utf8', -1);");
    session.execute(&s).wait().unwrap();
    let query = stmt!("SELECT i32 FROM examples.basic WHERE key = 'utf8';");
    let result = session.execute(&query).wait().unwrap();
    let row = result.iter().next().unwrap();
    let err = row
        .get_column(0)
        .unwrap()
        .get_string()
        .expect_err("Should have failed");
    println!("Got error {} kind {:?}", err, err.kind());
    match *err.kind() {
        ErrorKind::InvalidUtf8(_) => (),
        ref k => panic!("Unexpected error kind {}", k),
    }

    // NUL error
    let mut query = stmt!("SELECT ? from system_schema.tables;");
    let err = query
        .bind(0, "safe\0nasty!")
        .expect_err("Should have failed!");
    println!("Got error {} kind {:?}", err, err.kind());
    match *err.kind() {
        ErrorKind::StringContainsNul(_) => (),
        ref k => panic!("Unexpected error kind {}", k),
    }
}

#[test]
fn test_result() {
    let query = stmt!("SELECT * FROM system_schema.tables;");
    let session = help::create_test_session();
    let result = session.execute(&query).wait().unwrap();

    assert_eq!("keyspace_name", result.column_name(0).unwrap());
    assert_eq!("table_name", result.column_name(1).unwrap());
}

#[test]
fn test_statement_timeout() {
    let mut query = stmt!("SELECT * FROM system_schema.tables;");
    query.set_statement_request_timeout(Some(Duration::milliseconds(30000 as i64)));
    let session = help::create_test_session();
    let result = session.execute(&query).wait().unwrap();

    assert_eq!("keyspace_name", result.column_name(0).unwrap());
    assert_eq!("table_name", result.column_name(1).unwrap());
}

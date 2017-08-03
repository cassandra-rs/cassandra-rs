#[macro_use(stmt)]
extern crate cassandra_cpp;
extern crate futures;

mod help;

use cassandra_cpp::*;
use futures::Future;


#[derive(Debug,PartialEq,Clone,Copy)]
struct Basic {
    bln: bool,
    flt: f32,
    dbl: f64,
    i32: i32,
    i64: i64,
}

/// Create the table for basic testing.
fn create_basic_table(session: &Session) {
    let table_statement = &stmt!("CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt \
                                  float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));");
    session.execute(table_statement).wait().unwrap();
    let truncate_statement = &stmt!("TRUNCATE examples.basic;");
    session.execute(truncate_statement).wait().unwrap();
}

fn insert_into_basic(session: &Session, key: &str, basic: &Basic) -> Result<CassResult> {
    let mut statement = stmt!("INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);");
    statement.bind(0, key)?;
    statement.bind(1, basic.bln)?;
    statement.bind(2, basic.flt)?;
    statement.bind(3, basic.dbl)?;
    statement.bind(4, basic.i32)?;
    statement.bind(5, basic.i64)?;
    session.execute(&statement).wait()
}

const SELECT_QUERY: &str = "SELECT * FROM examples.basic WHERE key = ?";

fn select_from_basic(session: &Session, key: &str) -> Result<Option<Basic>> {
    let mut statement = stmt!(SELECT_QUERY);
    statement.bind_string(0, key)?;
    let result = session.execute(&statement).wait()?;
    println!("Result: \n{:?}\n", result);
    match result.first_row() {
        None => Ok(None),
        Some(row) => {
            Ok(Some(Basic {
                bln: row.get(1)?,
                dbl: row.get(2)?,
                flt: row.get(3)?,
                i32: row.get(4)?,
                i64: row.get(5)?,
            }))
        }
    }
}

fn select_from_basic_prepared(session: &Session, prepared: &PreparedStatement, key: &str, basic: &mut Basic) -> Result<()> {
    let mut statement = prepared.bind();
    statement.bind_string(0, key)?;
    let future = session.execute(&statement);
    let result = future.wait()?;
    println!("{:?}", result);
    for row in result.iter() {
        basic.bln = row.get(1)?;
        basic.dbl = row.get(2)?;
        basic.flt = row.get(3)?;
        basic.i32 = row.get(4)?;
        basic.i64 = row.get(5)?;
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
    session.execute(&s).wait().expect_err("Should cleanly return an error");
}

#[test]
fn test_basic_round_trip() {
    let session = help::create_test_session();
    help::create_example_keyspace(&session);
    create_basic_table(&session);

    let input = Basic {
        bln: true,
        flt: 0.001f32,
        dbl: 0.0002f64,
        i32: 1,
        i64: 2,
    };

    insert_into_basic(&session, "test", &input).unwrap();
    let output = select_from_basic(&session, "test").unwrap().expect("no output from select");

    println!("{:?}", input);
    println!("{:?}", output);

    assert!(input == output);
}

#[test]
fn test_prepared_round_trip() {
    let session = help::create_test_session();
    help::create_example_keyspace(&session);
    create_basic_table(&session);

    let input = Basic {
        bln: true,
        flt: 0.001f32,
        dbl: 0.0002f64,
        i32: 1,
        i64: 2,
    };
    let mut output = Basic {
        bln: false,
        flt: 0f32,
        dbl: 0f64,
        i32: 0,
        i64: 0,
    };

    println!("Basic insertions");
    insert_into_basic(&session, "prepared_test", &input).unwrap();
    println!("Preparing");
    let prepared = session.prepare(SELECT_QUERY).unwrap().wait().expect("prepared");
    select_from_basic_prepared(&session, &prepared, "prepared_test", &mut output).unwrap();
    assert_eq!(input, output, "Input:  {:?}\noutput: {:?}", &input, &output);
}

#[test]
fn test_null_retrieval() {
    let session = help::create_test_session();
    help::create_example_keyspace(&session);
    create_basic_table(&session);

    // Insert a partial row.
    let partial = stmt!("INSERT INTO examples.basic (key, bln, flt) VALUES ('vacant', true, 3.14);");
    session.execute(&partial).wait().expect("insert");

    // Read the whole row.
    let query = stmt!("SELECT key, bln, flt, dbl, i32, i64 FROM examples.basic WHERE key = 'vacant';");
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
    c.get_i32().expect_err("should be null");
    assert!(c.is_null());

    let c = row.get_column(5).expect("should be present");
    c.get_i64().expect_err("should be null");
    assert!(c.is_null());
}

#[test]
fn test_null_insertion() {
    let session = help::create_test_session();
    help::create_example_keyspace(&session);
    create_basic_table(&session);

    // Insert some explicit nulls.
    let mut s = stmt!("INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);");
    s.bind(0, "shrdlu").unwrap();
    s.bind(1, false).unwrap();
    s.bind_null(2).unwrap();
    s.bind(3, 2.72f64).unwrap();
    // deliberately omit 4 - this should be equivalent to binding null
    s.bind_null(5).unwrap();
    session.execute(&s).wait().unwrap();

    // Read them back.
    let s = stmt!("SELECT key, bln, flt, dbl, i32, i64 FROM examples.basic WHERE key = 'shrdlu';");
    let result = session.execute(&s).wait().expect("select");
    assert_eq!(1, result.row_count());
    let row = result.first_row().unwrap();

    assert!(!row.get_column(0).unwrap().is_null());
    assert!(!row.get_column(1).unwrap().is_null());
    assert!(row.get_column(2).unwrap().is_null());
    assert!(!row.get_column(3).unwrap().is_null());
    assert!(row.get_column(4).unwrap().is_null());
    assert!(row.get_column(5).unwrap().is_null());
}

/// Check for a needle in a haystack, and fail if not present.
fn assert_contains(haystack: String, needle: &str) {
    assert!(haystack.contains(needle), "assert_contains: `{}` not found in `{}`", needle, &haystack);
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
    assert_contains(column_debug, r#"{"class" => "org.apache.cassandra.db.compaction"#);

    let column_display = format!("{}", compaction_col);
    println!("Column display: {}", &column_display);
    assert_contains(column_display, r#"{class => org.apache.cassandra.db.compaction"#);

    // Check retrieving a string and a str.
    let keyspace_col = row.get_column_by_name("keyspace_name").unwrap();
    let str: &str = keyspace_col.get_str().unwrap();
    println!("Str is {}", str);
    assert!(str.len() > 0, "empty str");
    let str: String = keyspace_col.get_string().unwrap();
    println!("String is {}", str);
    assert!(str.len() > 0, "empty string");

    // Check invalid retrieval type.
    keyspace_col.get_map().expect_err("Should fail with invalid value type");
}

#[test]
fn test_error_reporting() {
    let session = help::create_test_session();

    // Simple error.
    let mut query = stmt!("SELECT * from system_schema.tables;");
    query.set_consistency(Consistency::THREE).unwrap();  // assuming we only have one node, this must fail
    let err = session.execute(&query).wait().expect_err("Should have failed!");
    println!("Got error {} kind {:?}", err, err.kind());
    match *err.kind() {
        ErrorKind::CassError(CassErrorCode::LIB_NO_HOSTS_AVAILABLE, _) => (),
        ref k => panic!("Unexpected error kind {}", k),
    }

    // Detailed error.
    let query = stmt!("SELECT gibberish from system_schema.tables;");
    let err = session.execute(&query).wait().expect_err("Should have failed!");
    println!("Got error {} kind {:?}", err, err.kind());
    match *err.kind() {
        ErrorKind::CassErrorResult(CassErrorCode::SERVER_INVALID_QUERY, _, _, -1, -1, -1, _, _, _, _, _) => (),
        ref k => panic!("Unexpected error kind {}", k),
    }

    // UTF-8 error
    let query = stmt!("SELECT (blob)0xffff from system_schema.tables;");
    let result = session.execute(&query).wait().unwrap();
    let row = result.iter().next().unwrap();
    let err = row.get_column(0).unwrap().get_string().expect_err("Should have failed");
    println!("Got error {} kind {:?}", err, err.kind());
    match *err.kind() {
        ErrorKind::InvalidUtf8(_) => (),
        ref k => panic!("Unexpected error kind {}", k),
    }

    // NUL error
    let mut query = stmt!("SELECT ? from system_schema.tables;");
    let err = query.bind(0, "safe\0nasty!").expect_err("Should have failed!");
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
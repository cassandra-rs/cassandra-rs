extern crate cassandra;
extern crate cql_bindgen;

use cassandra::*;

use cql_bindgen::cass_iterator_next;

static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE examples WITH replication = { \'class\': \
                                       \'SimpleStrategy\', \'replication_factor\': \'3\' };";
static CREATE_TABLE:&'static str = "CREATE TABLE examples.schema_meta (key text, value bigint, \
                                    PRIMARY KEY (key));";

unsafe fn print_keyspace(session: &mut Session, keyspace: &str) -> Result<(), CassError> {
    let schema = session.get_schema();
    let keyspace_meta = schema.get_keyspace(keyspace);
    try!(print_schema_meta(&keyspace_meta, 0));
    Ok(())
}

unsafe fn print_table(session: &mut Session,
                      keyspace: &str,
                      table: &str)
                      -> Result<(), CassError> {
    let keyspace_meta = session.get_schema().get_keyspace(keyspace);
    let table_meta = keyspace_meta.get_entry(table);
    try!(print_schema_meta(&table_meta, 0));
    Ok(())
}


fn main() {
    unsafe {
        let mut cluster = Cluster::new();
        cluster.set_contact_points("127.0.0.1").unwrap();
        match Session::new().connect(&mut cluster).wait() {
            Ok(mut session) => {
                let _ = session.execute_statement(&Statement::new(CREATE_KEYSPACE,0));
                print_keyspace(&mut session, "examples").unwrap();
                let _ = session.execute_statement(&Statement::new(CREATE_TABLE,0));
                print_table(&mut session, "examples", "schema_meta").unwrap();
                session.close().wait().unwrap();
            }
            Err(err) => println!("Unable to connect: '{:?}'\n", err),
        }
    }
}


fn print_indent(indent: u32) {
    for _ in 0..indent {
        print!("\t");
    }
}

unsafe fn print_schema_value(value: &CassValue) {

    match value.get_type() {
        ValueType::INT => {
            print!("{}", value.get_int32().unwrap());
        }

        ValueType::BOOLEAN => {
            print!("{}",value.get_bool().unwrap());
        }

        ValueType::DOUBLE => {
            print!("{}", value.get_double().unwrap());
        }

        ValueType::TEXT | ValueType::ASCII | ValueType::VARCHAR => {
            print!("{:?}", value            );
        }

        ValueType::UUID => {
            print!("{:?}", value.get_uuid().unwrap() /*us*/);
        }

        ValueType::LIST => {
            print_schema_list(value);
        }

        ValueType::MAP => {
            print_schema_map(value);
        }
        _ => {
            if value.is_null() {
                print!("null");
            } else {
                print!("<unhandled type>");
            }
        }
    }
}

unsafe fn print_schema_list(value: &CassValue) {
    let mut is_first = true;
    print!("[ ");
    for item in value.as_set_iterator().unwrap() {
        if is_first {
            print!(", ");
            is_first = false;
        }
        print_schema_value(&item);

    }
    print!(" ]");
}

unsafe fn print_schema_map(value: &CassValue) {
    let mut is_first = true;
    print!("[[ ");
    for (key,value) in value.as_map_iterator().unwrap() {
        if !is_first {
            print!(", ");
            is_first = false;
        }
        print_schema_value(&key);
        print!(" : ");
        print_schema_value(&value);
    }
    print!(" ]]");
}

unsafe fn print_schema_meta_field(field: &SchemaMetaField, indent: u32) {
    print_indent(indent);
    print!("{:?} : ", field.get_name());
    print_schema_value(&field.get_value());
    println!("");
}

unsafe fn print_schema_meta_fields(meta: &SchemaMeta, indent: u32) {
    let mut fields = meta.fields_from_schema_meta();

    while cass_iterator_next(fields.0) > 0 {
        print_schema_meta_field(&SchemaMetaField(fields.get_schema_meta_field().0), indent);
    }
}

unsafe fn print_schema_meta_entries(meta: &SchemaMeta, indent: u32) -> Result<(), CassError> {
    let mut entries = meta.iterator();

    while cass_iterator_next(entries.0) > 0 {
        try!(print_schema_meta(&SchemaMeta(entries.get_schema_meta().0), indent));
    }
    Ok(())
}

unsafe fn print_schema_meta(meta: &SchemaMeta, indent: u32) -> Result<(), CassError> {
    print_indent(indent);
    match try!(meta.get_type()) {
        SchemaMetaType::KEYSPACE => {
            println!("Keyspace {:?}",  meta.get_field("keyspace_name").get_value());
            print_schema_meta_fields(meta, indent + 1);
            println!("");
            try!(print_schema_meta_entries(meta, indent + 1));
            Ok(())
        }

        SchemaMetaType::TABLE => {
            println!("Table {:?}", meta.get_field("columnfamily_name").get_value());
            print_schema_meta_fields(meta, indent + 1);
            println!("");
            try!(print_schema_meta_entries(meta, indent + 1));
            Ok(())
        }

        SchemaMetaType::COLUMN => {
            println!("Column {:?}", meta.get_field("column_name").get_name());
            print_schema_meta_fields(meta, indent + 1);
            println!("");
            Ok(())
        }
    }
}

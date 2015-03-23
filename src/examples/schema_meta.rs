extern crate cql_ffi;
extern crate cql_bindgen;

use cql_ffi::*;

use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_schema_meta_field;
use cql_bindgen::cass_iterator_get_schema_meta;
use cql_bindgen::cass_iterator_fields_from_schema_meta;
use cql_bindgen::cass_iterator_from_schema_meta;

static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };";
static CREATE_TABLE:&'static str = "CREATE TABLE examples.schema_meta (key text, value bigint, PRIMARY KEY (key));";

unsafe fn print_keyspace(session:&mut CassSession, keyspace:&str) {
    let schema = session.get_schema();
    let keyspace_meta = schema.get_keyspace(keyspace);
    print_schema_meta(&keyspace_meta, 0);
}

unsafe fn print_table(session: &mut CassSession, keyspace:&str, table:&str) {
    let keyspace_meta = session.get_schema().get_keyspace(keyspace);
    let table_meta = keyspace_meta.get_entry(table);
    print_schema_meta(&table_meta, 0);
}


fn main() {unsafe{
    let mut cluster = CassCluster::new().set_contact_points("127.0.0.1").unwrap();
    match CassSession::new().connect(&mut cluster).wait() {
        Ok(mut session) => {
            let _ = session.execute_statement(&CassStatement::new(CREATE_KEYSPACE,0));
            print_keyspace(&mut session, "examples");
            let _ = session.execute_statement(&CassStatement::new(CREATE_TABLE,0));
            print_table(&mut session, "examples", "schema_meta");
            session.close().wait().unwrap();
        },
        Err(err) => println!("Unable to connect: '{:?}'\n", err)
        
    }
}}


fn print_indent(indent:u32) {
    for _ in 0..indent {
        print!("\t");
    }
}

unsafe fn print_schema_value(value:&CassValue) {

    match value.get_type() {
        CassValueType::INT => {
            print!("{}", value.get_int32().unwrap());
        }

        CassValueType::BOOLEAN => {
            print!("{}",value.get_bool().unwrap());
        }   
    
        CassValueType::DOUBLE => {
            print!("{}", value.get_double().unwrap());
        }

        CassValueType::TEXT|CassValueType::ASCII|CassValueType::VARCHAR => {
            print!("{:?}", value.get_string());
        }
    
        CassValueType::UUID => {
            print!("{:?}", value.get_uuid().unwrap() /*us*/);
        }
    
        CassValueType::LIST => {
            print_schema_list(value);
        }

        CassValueType::MAP => {
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

unsafe fn print_schema_list(value:&CassValue) {
    let mut iterator = value.as_collection_iterator();
    let mut is_first = cass_true;

    print!("[ ");
    while cass_iterator_next(iterator.0) > 0 {
        if !is_first > 0 {print!(", ")};
        print_schema_value(&iterator.get_value());
        is_first = cass_false;
    }
    print!(" ]");
}

unsafe fn print_schema_map(value:&CassValue) {
    let mut is_first = cass_true;

    print!("[[ ");
    for (key,value) in value.as_map_iterator() {
        if !is_first > 0 {} print!(", ");
        print_schema_value(&key);
        print!(" : ");
        print_schema_value(&value);
        is_first = cass_false;
    }
    print!(" ]]");
}

unsafe fn print_schema_meta_field(field:&CassSchemaMetaField, indent:u32) {
    let name = field.get_name();
    let value = field.get_value();

    print_indent(indent);
  
    print!("{:?}", name);
    print_schema_value(&value);
    print!("\n");
}

unsafe fn print_schema_meta_fields(meta:&CassSchemaMeta, indent:u32) {
    let fields = cass_iterator_fields_from_schema_meta(meta.0);

    while cass_iterator_next(fields) > 0 {
        print_schema_meta_field(&CassSchemaMetaField(cass_iterator_get_schema_meta_field(fields)), indent);
    }
}

unsafe fn print_schema_meta_entries(meta:&CassSchemaMeta, indent:u32) {
    let entries = cass_iterator_from_schema_meta(meta.0);

    while cass_iterator_next(entries) > 0 {
        print_schema_meta(&CassSchemaMeta(cass_iterator_get_schema_meta(entries)), indent);
    }
}


unsafe fn print_schema_meta(meta:&CassSchemaMeta, indent:u32) {
    print_indent(indent);

    match meta.get_type().unwrap() {
        CassSchemaMetaType::KEYSPACE => {
            let KS_NAME = "keyspace_name";
            let field = meta.get_field(KS_NAME);
            let value = field.get_value();
            let name = value.get_string();

            println!("Keyspace {:?}", name);
            print_schema_meta_fields(meta, indent + 1);
            println!("");
            print_schema_meta_entries(meta, indent + 1);
        }

        CassSchemaMetaType::TABLE => {
            let CF_NAME = "columnfamily_name";
            let field = meta.get_field(CF_NAME);
            let name = field.get_value().get_string();      
            println!("Table {:?}", name);
            print_schema_meta_fields(meta, indent + 1);
            println!("");
            print_schema_meta_entries(meta, indent + 1);
        }
    
        CassSchemaMetaType::COLUMN => {
            let COLUMN_NAME = "column_name";
            let field = meta.get_field(COLUMN_NAME);
            let name = field.get_name();
            println!("Column {:?}", name);
            print_schema_meta_fields(meta, indent + 1);
            println!("");
        }
    }
}

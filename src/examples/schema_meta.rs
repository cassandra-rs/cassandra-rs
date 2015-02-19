extern crate cql_ffi;

use cql_ffi::*;

static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };";
static CREATE_TABLE:&'static str = "CREATE TABLE examples.schema_meta (key text, value bigint, PRIMARY KEY (key));";

unsafe fn print_keyspace(session:&mut CassSession, keyspace:&str) {
    let schema = session.get_schema();
    let keyspace_meta = schema.get_keyspace(keyspace);
    //FIXME
    //~ if keyspace_meta.is_null() {
        //~ println!("Unable to find \"{}\" keyspace in the schema metadata\n", keyspace);
    //~ } else {
        print_schema_meta(&keyspace_meta, 0);
    //~ }
}

unsafe fn print_table(session: &mut CassSession, keyspace:&str, table:&str) {
    let keyspace_meta = session.get_schema().get_keyspace(keyspace);
    //~ match keyspace_meta.is_null() {
        //~ true => println!("Unable to find \"{}\" keyspace in the schema metadata\n", keyspace),
        //~ false => {
            let table_meta = keyspace_meta.get_entry(table);
            //~ match table_meta.is_null() {
                //~ true => {
                    print_schema_meta(&table_meta, 0);
                //~ },
                //~ false => println!("Unable to find \"{}\" table in the schema metadata\n", keyspace)
            //~ }
        //~ }
    //~ }
}

fn main() {unsafe{
    let mut cluster = CassCluster::new();
    match CassSession::new().connect(&mut cluster).wait() {
        Ok(mut session) => {
            let _ = session.execute_statement(&CassStatement::new(CREATE_KEYSPACE,0));
            print_keyspace(&mut session, "examples");
            let _ = session.execute_statement(&CassStatement::new(CREATE_TABLE,0));
            print_table(&mut session, "examples", "schema_meta");
            /* Close the session */
            session.close().wait().unwrap();
        },
        Err(err) => println!("Unable to connect: '{:?}'\n", err)
        
    }
}}

fn print_schema_value(value:&CassValue) {unsafe{
    use cql_ffi::CassValueType::*;
    match value.get_type() {
        CassValueType::INT  => {
            println!("{}", value.get_int32().unwrap());
        },
        BOOLEAN => {
            let cbool = value.get_bool();
            println!("{:?}", cbool.unwrap());
        },
        DOUBLE => {
            let cdouble = value.get_double();
            println!("{:?}", cdouble);
        },
        //~ TEXT|ASCII|VARCHAR => {
            //~ let cstring = cassvalue2cassstring(value);
            //~ println!("\"{:?}\"", cstring);
        //~ },
        UUID => {
            match value.get_uuid() {
                Ok(uuid) => {
                    println!("{:?}", uuid);
                },
                Err(err) => panic!(err)
            }
        },
        LIST => {
            print_schema_list(value);
        },
        MAP => {
            print_schema_map(value)
        },
        _ => {
            println!("<unhandled type>");
        }
    }
}}

fn print_schema_list(value:&CassValue) {unsafe{
    let mut iterator = value.as_collection_iterator();;
//    let mut is_first = true;
    //printf("[ ");
    while iterator.next() {
//        if (!is_first) {println!(", ");}
        print_schema_value(&iterator.get_value());
 //       is_first = if cass_false > 0 {true} else {false};
    }
    //printf(" ]");
}}

fn print_schema_map(value:&CassValue) {unsafe{
    let mut iterator = value.as_collection_iterator();
    //let mut is_first = cass_true;
    //printf("{ ");
    while iterator.next() {
        //if (!is_first) printf(", ");
        print_schema_value(&iterator.get_map_key());
        //printf(" : ");
        print_schema_value(&iterator.get_map_value());
        //is_first = cass_false;
    }
    //printf(" }");
}}

fn print_schema_meta_field(field:&CassSchemaMetaField, _indent:i32) {unsafe{
    let (name,value) = (field.get_name(),field.get_value());
    //print_indent(indent);
    println!("{:?}: ", ToString::to_string(&name));
    print_schema_value(&value);
    //printf("\n");
}}

fn print_schema_meta_fields(meta:&CassSchemaMeta, indent:i32) {unsafe{
    let mut fields = meta.fields_iterator();
    while fields.next() {
        print_schema_meta_field(&fields.get_schema_meta_field(), indent);
    }
}}

fn print_schema_meta_entries(meta:&CassSchemaMeta, indent:i32) {unsafe{
    let mut entries = meta.iterator();
    while entries.next() {
        print_schema_meta(&entries.get_schema_meta(), indent);
    }
}}

fn print_schema_meta(meta:&CassSchemaMeta, indent:i32) {unsafe{
    //print_indent(indent);
    match meta.get_type().unwrap() {
        CassSchemaMetaType::KEYSPACE => {
            let field = meta.get_field("keyspace_name");
            //~ if !field.is_null() {
                let name = &field.get_value();
                println!("Keyspace \"{:?}\":", ToString::to_string(&name.get_string().unwrap()));
            //~ }
            print_schema_meta_fields(meta, indent + 1);
            //printf("\n");
            print_schema_meta_entries(meta, indent + 1);
        },
        CassSchemaMetaType::TABLE => {
            let field = meta.get_field("columnfamily_name");
            let name = &field.get_value();
            println!("Table \"{:?}\":", ToString::to_string(&name.get_string().unwrap()));
            print_schema_meta_fields(meta, indent + 1);
            //printf("\n");
            print_schema_meta_entries(meta, indent + 1);
        },
        CassSchemaMetaType::COLUMN => {
            let field = meta.get_field("column_name");
            //~ if !field.is_null() {
                let name = &field.get_value();
                println!("Column \"{:?}\":\n", ToString::to_string(&name.get_string().unwrap()));
            //~ }
            print_schema_meta_fields(meta, indent + 1);
            //printf("\n");
        }
    }
}}

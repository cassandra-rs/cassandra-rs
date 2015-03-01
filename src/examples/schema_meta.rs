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
        print_schema_meta(&keyspace_meta);
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
                    print_schema_meta(&table_meta);
                //~ },
                //~ false => println!("Unable to find \"{}\" table in the schema metadata\n", keyspace)
            //~ }
        //~ }
    //~ }
}

fn main() {unsafe{
    let mut cluster = CassCluster::new().set_contact_points("127.0.0.1").unwrap();
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


//~ fn print_schema_meta_fields(meta:&CassSchemaMeta, indent:i32) {
    //~ for value in  meta.fields_iterator() {
        //~ println!("{:?}",value);
    //~ }
//~ }

fn print_schema_meta_entries(meta:&CassSchemaMeta) {
    for value in meta.iterator() {
        println!("{:?}",value);
    }
}

fn print_schema_meta(meta:&CassSchemaMeta) {
    //print_indent(indent);
    match meta.get_type().unwrap() {
        CassSchemaMetaType::KEYSPACE => {
            let field = meta.get_field("keyspace_name");
            //~ if !field.is_null() {
                let name = &field.get_value();
                println!("Keyspace \"{:?}\":", ToString::to_string(&name.get_string().unwrap()));
            //~ }
            for value in  meta.fields_iterator() {
                println!("{:?}",value);
            }          
            print_schema_meta_entries(meta);
        },
        CassSchemaMetaType::TABLE => {
            let field = meta.get_field("columnfamily_name");
            let name = &field.get_value();
            println!("Table \"{:?}\":", ToString::to_string(&name.get_string().unwrap()));
            for value in  meta.fields_iterator() {
                println!("{:?}",value);
            }          
            //printf("\n");
            print_schema_meta_entries(meta);
        },
        CassSchemaMetaType::COLUMN => {
            let field = meta.get_field("column_name");
            //~ if !field.is_null() {
                let name = &field.get_value();
                println!("Column \"{:?}\":\n", ToString::to_string(&name.get_string().unwrap()));
            //~ }
            for value in  meta.fields_iterator() {
                println!("{:?}",value);
            }          
            //printf("\n");
        }
    }
}

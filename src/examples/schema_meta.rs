extern crate cql_ffi;

use std::slice;

use cql_ffi::*;

//fn print_schema_meta(const CassSchemaMeta* meta, int indent);

unsafe fn print_keyspace(session:&mut CassSession, keyspace:&str) {
    let schema = cass_session_get_schema(session);
    let keyspace_meta = cass_schema_get_keyspace(schema, str2ref(keyspace));
    if keyspace_meta.is_null() {
        println!("Unable to find \"{}\" keyspace in the schema metadata\n", keyspace);
    } else {
        print_schema_meta(&*keyspace_meta, 0)
    }
    cass_schema_free(schema);
}

unsafe fn print_table(session: &mut CassSession, keyspace:&str, table:&str) {
    let schema = cass_session_get_schema(session);
    let keyspace_meta = cass_schema_get_keyspace(schema, str2ref(keyspace));
    match keyspace_meta.is_null() {
        true => println!("Unable to find \"{}\" keyspace in the schema metadata\n", keyspace),
        false => {
            let table_meta = cass_schema_meta_get_entry(keyspace_meta, str2ref(table));
            match table_meta.is_null() {
                true => {
                    print_schema_meta(&*table_meta, 0);
                },
                false => println!("Unable to find \"{}\" table in the schema metadata\n", keyspace)
            }
        }
    }
    cass_schema_free(schema);
}


unsafe fn print_error(future:&mut CassFuture) {
    let message = cass_future_error_message(future);
    let message = slice::from_raw_buf(&message.data,message.length as usize);
    println!("Error: {:?}", message);
}

fn execute_query(session: &mut CassSession, query: &str) -> CassError {unsafe{
    let query=str2cass_string(query);

    let statement = cass_statement_new(query, 0);
    let future = &mut *cass_session_execute(session, statement);
    cass_future_wait(future);
    let rc = cass_future_error_code(future);
    match rc {
        CassError::CASS_OK => {},
        _ => print_error(future)
    }
    cass_future_free(future);
    cass_statement_free(statement);
    return rc;
}}

fn main() {unsafe{
    let  cluster = cass_cluster_new();
    let session = cass_session_new();
    cass_cluster_set_contact_points(cluster, str2ref("127.0.0.1"));
    let connect_future = cass_session_connect(session, cluster);
    match cass_future_error_code(connect_future) {
        CassError::CASS_OK => {
            execute_query(&mut*session, "CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };");
            print_keyspace(&mut*session, "examples");
            execute_query(&mut*session, "CREATE TABLE examples.schema_meta (key text, value bigint, PRIMARY KEY (key));");
            print_table(&mut*session, "examples", "schema_meta");
            /* Close the session */
            let close_future = cass_session_close(&mut*session);
            cass_future_wait(close_future);
            cass_future_free(close_future);
        },
        _ => {
        /* Handle error */
            let message = cass_future_error_message(connect_future);
            println!("Unable to connect: '{:?}'\n", message);
        }
    }
    cass_future_free(connect_future);
    cass_cluster_free(cluster);
    cass_session_free(session);
}}

fn print_schema_value(value:&CassValue) {unsafe{
//~ cass_int32_t i;
//~ cass_bool_t b;
//~ cass_double_t d;
//~ CassString s;
//~ CassUuid u;
//    char us[CASS_UUID_STRING_LENGTH];
    let cass_value_type = cass_value_type(value);
    match cass_value_type {
        CassValueType::CASS_VALUE_TYPE_INT  => {
            let cint = &mut 0i32;
            cass_value_get_int32(value, cint);
            println!("{}", cint);
        },
        CassValueType::CASS_VALUE_TYPE_BOOLEAN => {
            let mut cbool = 0u32;
            cass_value_get_bool(value, &mut cbool);
            println!("{:?}", if cbool > 0u32 {"true"} else {"false"});
        },
        CassValueType::CASS_VALUE_TYPE_DOUBLE => {
            let mut cdouble = 0f64;
            cass_value_get_double(value, &mut cdouble);
            println!("{:?}", cdouble);
        },
        CassValueType::CASS_VALUE_TYPE_TEXT|CassValueType::CASS_VALUE_TYPE_ASCII|CassValueType::CASS_VALUE_TYPE_VARCHAR => {
            let cstring = cassvalue2cassstring(value);
            println!("\"{:?}\"", cstring);
        },
        CassValueType::CASS_VALUE_TYPE_UUID => {
            let uuid = cassvalue2cassuuid(value);
            let uuid_str = cassuuid2string(uuid.unwrap());
            println!("{:?}", uuid_str);
        },
        CassValueType::CASS_VALUE_TYPE_LIST => {
            print_schema_list(value);
        },
        CassValueType::CASS_VALUE_TYPE_MAP => {
            print_schema_map(value)
        },
        _ => {
            println!("<unhandled type>");
        }
    }
}}

fn print_schema_list(value:&CassValue) {unsafe{
    let iterator = cass_iterator_from_collection(value);
    let mut is_first = true;
    //printf("[ ");
    while cass_iterator_next(iterator) > 0 {
//        if (!is_first) {println!(", ");}
        print_schema_value(&*cass_iterator_get_value(&mut*iterator));
        is_first = if cass_false > 0 {true} else {false};
    }
    //printf(" ]");
    cass_iterator_free(iterator);
}}

fn print_schema_map(value:&CassValue) {unsafe{
    let iterator = cass_iterator_from_map(value);
    let mut is_first = cass_true;
    //printf("{ ");
    while cass_iterator_next(iterator) > 0 {
        //if (!is_first) printf(", ");
        print_schema_value(&*cass_iterator_get_map_key(&mut*iterator));
        //printf(" : ");
        print_schema_value(&*cass_iterator_get_map_value(&mut*iterator));
        is_first = cass_false;
    }
    //printf(" }");
    cass_iterator_free(iterator);
}}

fn print_schema_meta_field(field:&CassSchemaMetaField, indent:i32) {unsafe{
    let name = cass_schema_meta_field_name(field);
    let value = cass_schema_meta_field_value(field);
    //print_indent(indent);
    println!("{:?}: ", name);
    print_schema_value(&*value);
    //printf("\n");
}}

fn print_schema_meta_fields(meta:&CassSchemaMeta, indent:i32) {unsafe{
    let fields = cass_iterator_fields_from_schema_meta(meta);
    while cass_iterator_next(fields) > 0 {
        print_schema_meta_field(&*cass_iterator_get_schema_meta_field(&mut*fields), indent);
    }
    cass_iterator_free(fields);
}}

fn print_schema_meta_entries(meta:&CassSchemaMeta, indent:i32) {unsafe{
    let entries = cass_iterator_from_schema_meta(meta);
    while cass_iterator_next(entries) > 0 {
        print_schema_meta(&*cass_iterator_get_schema_meta(entries), indent);
    }
    cass_iterator_free(entries);
}}

fn print_schema_meta(meta:&CassSchemaMeta, indent:i32) {unsafe{
    //print_indent(indent);
    match cass_schema_meta_type(meta) {
        CassSchemaMetaType::KEYSPACE => {
            let field = cass_schema_meta_get_field(meta, str2ref("keyspace_name"));
            if !field.is_null() {
                let name = cassvalue2cassstring(&*cass_schema_meta_field_value(field));
                println!("Keyspace \"{:?}\":", name);
            }
            print_schema_meta_fields(meta, indent + 1);
            //printf("\n");
            print_schema_meta_entries(meta, indent + 1);
        },
        CassSchemaMetaType::TABLE => {
            let field = cass_schema_meta_get_field(meta, str2ref("columnfamily_name"));
            let name = cassvalue2cassstring(&*cass_schema_meta_field_value(field));
            println!("Table \"{:?}\":", name);
            print_schema_meta_fields(meta, indent + 1);
            //printf("\n");
            print_schema_meta_entries(meta, indent + 1);
        },
        CassSchemaMetaType::COLUMN => {
            let field = cass_schema_meta_get_field(meta, str2ref("column_name"));
            if !field.is_null() {
                let name = cassvalue2cassstring(&*cass_schema_meta_field_value(field));
                println!("Column \"{:?}\":\n", name);
            }
            print_schema_meta_fields(meta, indent + 1);
            //printf("\n");
        }
    }
}}

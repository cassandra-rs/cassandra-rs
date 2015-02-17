extern crate cql_ffi;

use cql_ffi::*;

unsafe fn print_keyspace(session:&mut CassSession, keyspace:&str) {
    let schema = session.get_schema();
    let keyspace_meta = schema.get_keyspace(str2ref(keyspace));
    //FIXME
    //~ if keyspace_meta.is_null() {
        //~ println!("Unable to find \"{}\" keyspace in the schema metadata\n", keyspace);
    //~ } else {
        print_schema_meta(&keyspace_meta, 0);
    //~ }
}

unsafe fn print_table(session: &mut CassSession, keyspace:&str, table:&str) {
    let keyspace_meta = session.get_schema().get_keyspace(str2ref(keyspace));
    //~ match keyspace_meta.is_null() {
        //~ true => println!("Unable to find \"{}\" keyspace in the schema metadata\n", keyspace),
        //~ false => {
            let table_meta = keyspace_meta.get_entry(str2ref(table));
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
    let session = &mut CassSession::new();
    let connect_future = session.connect(&mut cluster);
    match session.connect(&mut cluster).wait() {
        Ok(_) => {
            let _ = session.execute_statement(&CassStatement::new("CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };",0));
            print_keyspace(session, "examples");
            let _ = session.execute_statement(&CassStatement::new("CREATE TABLE examples.schema_meta (key text, value bigint, PRIMARY KEY (key));",0));
            print_table(session, "examples", "schema_meta");
            /* Close the session */
            session.close().wait().unwrap();
        },
        _ => {
        /* Handle error */
            match connect_future.wait() {
                Ok(_) => {},
                Err(err) => println!("Unable to connect: '{:?}'\n", err)
            }
        }
    }
}}

fn print_schema_value(value:&CassValue) {unsafe{
    use cql_ffi::CassValueType::*;
    match value.get_type() {
        CassValueType::INT  => {
            let cint = &mut 0i32;
            let _ = value.get_int32(cint);
            println!("{}", cint);
        },
        BOOLEAN => {
            let mut cbool = 0u32;
            let _ = value.get_bool(&mut cbool);
            println!("{:?}", if cbool > 0u32 {"true"} else {"false"});
        },
        DOUBLE => {
            let mut cdouble = 0f64;
            let _ = value.get_double(&mut cdouble);
            println!("{:?}", cdouble);
        },
        //~ TEXT|ASCII|VARCHAR => {
            //~ let cstring = cassvalue2cassstring(value);
            //~ println!("\"{:?}\"", cstring);
        //~ },
        UUID => {
            match cassvalue2cassuuid(value) {
                Ok(uuid) => {
                    let uuid_str = cassuuid2string(uuid);
                    println!("{:?}", uuid_str);
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
            let field = meta.get_field(str2ref("keyspace_name"));
            //~ if !field.is_null() {
                let name = &field.get_value();
                println!("Keyspace \"{:?}\":", ToString::to_string(&name.get_string().unwrap()));
            //~ }
            print_schema_meta_fields(meta, indent + 1);
            //printf("\n");
            print_schema_meta_entries(meta, indent + 1);
        },
        CassSchemaMetaType::TABLE => {
            let field = meta.get_field(str2ref("columnfamily_name"));
            let name = &field.get_value();
            println!("Table \"{:?}\":", ToString::to_string(&name.get_string().unwrap()));
            print_schema_meta_fields(meta, indent + 1);
            //printf("\n");
            print_schema_meta_entries(meta, indent + 1);
        },
        CassSchemaMetaType::COLUMN => {
            let field = meta.get_field(str2ref("column_name"));
            //~ if !field.is_null() {
                let name = &field.get_value();
                println!("Column \"{:?}\":\n", ToString::to_string(&name.get_string().unwrap()));
            //~ }
            print_schema_meta_fields(meta, indent + 1);
            //printf("\n");
        }
    }
}}

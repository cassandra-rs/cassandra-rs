extern crate cql_ffi;
use cql_ffi::*;

static QUERY:&'static str = "SELECT keyspace_name FROM system.schema_keyspaces;";
static COL_NAME:&'static str = "keyspace_name";
static CONTACT_POINTS:&'static str = "127.0.0.1,127.0.0.2,127.0.0.3";

fn main() {unsafe{
    let cluster = &mut CassCluster::new()
                        .set_contact_points(CONTACT_POINTS)
                        .unwrap();
                        
    let mut session = CassSession::new();
    match session.connect(cluster).wait() {
        Ok(_) => {
            match session.execute(QUERY, 0).wait() {
                Ok(mut future) => {
                    let result = future.get_result();
                    let iter = &mut result.iter();
                    while iter.next() {
                        let row = iter.get_row();
                        let value:CassValue = row.get_column_by_name(COL_NAME);
                        let keyspace_name = value.get_string().unwrap();
                        println!("keyspace_name = {:?}", keyspace_name);
                    }
                },
                Err(err) => println!("Unable to run query: '{:?}'\n", err)              
            }
            session.close().wait().unwrap();
        },
        Err(err) => println!("Unable to connect: '{:?}'", err)
    }
}}

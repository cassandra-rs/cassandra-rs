extern crate cql_ffi;

use cql_ffi::*;

fn main() {unsafe{
    /* Setup and connect to cluster */
    let cluster = CassCluster::new();
    let session = cass_session_new();
    CassCluster::set_contact_points(cluster, str2ref("127.0.0.1,127.0.0.2,127.0.0.3"));
    let connect_future = cass_session_connect(session, cluster);
    match CassFuture::error_code(connect_future) {
        Ok(_) => {
            /* Build statement and execute query */
            let query = str2cass_string("SELECT keyspace_name FROM system.schema_keyspaces;");
            let statement = CassStatement::new(query, 0);
            let result_future = cass_session_execute(session, statement);
            match CassFuture::error_code(result_future) {
                Ok(_) => {
                    /* Retrieve result set and iterator over the rows */
                    let result = CassFuture::get_result(result_future);
                    let rows = CassIterator::from_result(result);
                    while CassIterator::next(rows) > 0 {
                        let row = CassIterator::get_row(rows);
                        let value = CassIterator::get_column_by_name(row, str2ref("keyspace_name"));
                        let keyspace_name = cassvalue2cassstring(&*value);
                        println!("keyspace_name: '{:?}'", keyspace_name);
                    }
                    cass_result_free(result);
                    cass_iterator_free(rows);
                },
                _=> {
                    /* Handle error */
                    let message = cass_future_error_message(result_future);
                    println!("Unable to run query: '{:?}'\n", message);                
                }
            }
            cass_statement_free(statement);
            cass_future_free(result_future);
            /* Close the session */
            let close_future = cass_session_close(session);
            cass_future_wait(close_future);
            cass_future_free(close_future);
        },
        _ => {
            //~ /* Handle error */
            let message = cass_future_error_message(connect_future);
            println!("Unable to connect: '{:?}'", message);
        }
    }
    cass_future_free(connect_future);
    cass_cluster_free(cluster);
    cass_session_free(session);
}}

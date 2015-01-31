extern crate cql_ffi;

use cql_ffi::*;

fn main() {unsafe{
    /* Setup and connect to cluster */
    let cluster = cass_cluster_new();
    let session = cass_session_new();
    cass_cluster_set_contact_points(cluster, str2ref("127.0.0.1,127.0.0.2,127.0.0.3"));
    let connect_future = cass_session_connect(session, cluster);
    match cass_future_error_code(connect_future) {
        CassError::CASS_OK => {
            /* Build statement and execute query */
            let query = str2cass_string("SELECT keyspace_name FROM system.schema_keyspaces;");
            let statement = cass_statement_new(query, 0);
            let result_future = cass_session_execute(session, statement);
            match cass_future_error_code(result_future) {
                CassError::CASS_OK => {
                    /* Retrieve result set and iterator over the rows */
                    let result = cass_future_get_result(result_future);
                    let rows = cass_iterator_from_result(result);
                    while cass_iterator_next(rows) > 0 {
                        let row = cass_iterator_get_row(rows);
                        let value = cass_row_get_column_by_name(row, str2ref("keyspace_name"));
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

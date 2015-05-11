extern crate cql_ffi;

use std::slice;

use cql_ffi::*;

unsafe fn print_error(future:&mut CassFuture) {
    let message = future.error_message();
    let message = slice::from_raw_parts(&message.data,message.length as usize);
    println!("Error: {:?}", message);
}

unsafe fn create_cluster() -> *mut CassCluster {
    let cluster = CassCluster::new();
    cluster.set_contact_points(str2ref("127.0.0.1,127.0.0.2,127.0.0.3"));
    cluster 
}

unsafe fn connect_session(session:&mut CassSession, cluster:&mut CassCluster) -> CassError {
    let future:CassFuture = &mut session.connect(cluster);
    future.wait();
    future
}

//~ fn on_log(message:&CassLogMessage, data:data) {

    //~ println!("{}",
        //~ message.time_ms / 1000,
        //~ message->time_ms % 1000,
        //~ cass_log_level_string(message.severity),
        //~ message.file, message.line, message.function,
        //~ message.message);
//~ }

fn main() {
    //~ CassCluster* cluster = NULL;
    //~ CassSession* session = NULL;
    //~ CassFuture* close_future = NULL;
    //~ FILE* log_file = fopen("driver.log", "w+");
    //~ if (log_file == NULL) {
        //~ fprintf(stderr, "Unable to open log file\n");
    //~ }
/* Log configuration *MUST* be done before any other driver call */
    cass_log_set_level(CASS_LOG_INFO);
    cass_log_set_callback(on_log, log_file);
    cluster = create_cluster();
    session = cass_session_new();
    if (connect_session(session, cluster) != CASS_OK) {
        cass_cluster_free(cluster);
        cass_session_free(session);
        return -1;
    }
    close_future = cass_session_close(session);
    cass_future_wait(close_future);
    cass_future_free(close_future);
    cass_cluster_free(cluster);
    cass_session_free(session);
/* This *MUST* be the last driver call */
    cass_log_cleanup();
    fclose(log_file);
    return 0;
}

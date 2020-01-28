use std::slice;
use std::str::FromStr;

use cassandra::*;

fn print_error(future: &mut Future) {
    let message = future.error_message();
    println!("Error: {:?}", message);
}

unsafe fn create_cluster() -> Result<CassError, Cluster> {
    let cluster = Cluster::default();
    cluster.set_contact_points("127.0.0.1,127.0.0.2,127.0.0.3")?;
    cluster
}

unsafe fn connect_session(session: &mut Session, cluster: &mut Cluster) -> CassError {
    let future: Future = &mut session.connect(cluster);
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
    //~ Cluster* cluster = NULL;
    //~ Session* session = NULL;
    //~ Future* close_future = NULL;
    //~ FILE* log_file = fopen("driver.log", "w+");
    //~ if (log_file == NULL) {
    //~ fprintf(stderr, "Unable to open log file\n");
    //~ }
    /* Log configuration *MUST* be done before any other driver call */
    use CassLogLevel::*;
    cass_log_set_level(CASS_LOG_INFO);
    cass_log_set_callback(on_log, log_file);
    cluster = create_cluster();
    session = cass_session_new();
    if connect_session(session, cluster) != CASS_OK {
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

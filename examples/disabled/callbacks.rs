use cassandra::*;
use std::ffi::CString;
use std::slice;

//~ uv_mutex_t mutex;
//~ uv_cond_t cond;
//~ Future* close_future = NULL;
//~ UuidGen* uuid_gen = NULL;

fn wait_exit() {
    uv_mutex_lock(&mutex);
    while (close_future == NULL) {
        uv_cond_wait(&cond, &mutex);
    }
    uv_mutex_unlock(&mutex);
    cass_future_wait(close_future);
    cass_future_free(close_future);
}

fn signal_exit(session: &Session) {
    uv_mutex_lock(&mutex);
    close_future = cass_session_close(session);
    uv_cond_signal(&cond);
    uv_mutex_unlock(&mutex);
}

//~ void on_create_keyspace(Future* future, void* data);
//~ void on_create_table(Future* future, void* data);
//~ void on_insert(Future* future, void* data);
//~ void on_select(Future* future, void* data);
//~ void on_session_connect(Future* future, void* data);
//~ void on_session_close(Future* future, void* data);

unsafe fn print_error(future: &mut Future) {
    let message = cass_future_error_message(future);
    let message = slice::from_raw_buf(&message.data, message.length as usize);
    println!("Error: {:?}", message);
}

unsafe fn create_cluster() -> *mut Cluster {
    let cluster = cass_cluster_new();
    cass_cluster_set_contact_points(cluster, str2ref("127.0.0.1,127.0.0.2,127.0.0.3"));
    cluster
}

fn connect_session(session: Session, cluster: &Cluster, callback: FutureCallback) {
    let future = cass_session_connect_keyspace(session, cluster, "examples");
    cass_future_set_callback(future, callback, session);
    cass_future_free(future);
}

fn execute_query(session: Session, query: &str, callback: FutureCallback) {
    let statement = cass_statement_new(cass_string_init(query), 0);
    let future = cass_session_execute(session, statement);
    cass_future_set_callback(future, callback, session);
    cass_future_free(future);
    cass_statement_free(statement);
}

fn on_session_connect(future: Future, data: void) {
    let session: Session = data;
    let code = cass_future_error_code(future);
    if (code != CASS_OK) {
        print_error(future);
        uv_cond_signal(&cond);
        return;
    }
    execute_query(
        session,
        "CREATE KEYSPACE examples WITH replication = {\'class\': \'SimpleStrategy\', \
         \'replication_factor\': \'3\' };",
        on_create_keyspace,
    );
}

fn on_create_keyspace(future: Future, data: void) {
    let code = cass_future_error_code(future);
    if (code != CASS_OK) {
        print_error(future);
    }
    execute_query(
        data,
        "CREATE TABLE callbacks (key timeuuid PRIMARY KEY, value bigint)",
        on_create_table,
    );
}

fn on_create_table(future: Future, data: Session) {
    let insert_query = cass_string_init("INSERT INTO callbacks (key, value) VALUES (?, ?)");
    let code = cass_future_error_code(future);
    if (code != CASS_OK) {
        print_error(future);
    }
    statement = cass_statement_new(insert_query, 2);
    cass_uuid_gen_time(uuid_gen, &key);
    cass_statement_bind_uuid(statement, 0, key);
    cass_statement_bind_int64(statement, 1, cass_uuid_timestamp(key));
    insert_future = cass_session_execute(data, statement);
    cass_future_set_callback(insert_future, on_insert, data);
    cass_statement_free(statement);
    cass_future_free(insert_future);
}

fn on_insert(future: Future, data: Session) {
    let code = cass_future_error_code(future);
    if (code != CASS_OK) {
        print_error(future);
        signal_exit(data);
    } else {
        let select_query = cass_string_init("SELECT * FROM callbacks");
        let statement = cass_statement_new(select_query, 0);
        let select_future = cass_session_execute(data, statement);
        cass_future_set_callback(select_future, on_select, data);
        cass_statement_free(statement);
        cass_future_free(select_future);
    }
}

fn on_select(future: Future, data: Session) {
    let code = cass_future_error_code(future);
    if (code != CASS_OK) {
        print_error(future);
    } else {
        let result = cass_future_get_result(future);
        let iterator = cass_iterator_from_result(result);
        while (cass_iterator_next(iterator)) {
            //Uuid key;
            //char key_str[CASS_UUID_STRING_LENGTH];
            let value = 0u64;
            let row = cass_iterator_get_row(iterator);
            cass_value_get_uuid(cass_row_get_column(row, 0), &key);
            cass_uuid_string(key, key_str);
            cass_value_get_int64(cass_row_get_column(row, 1), &value);
            printf("%s, %llu\n", key_str, value);
        }
        cass_iterator_free(iterator);
        cass_result_free(result);
    }
    signal_exit(data);
}

fn main() {
    Cluster * cluster = create_cluster();
    Session * session = cass_session_new();
    uuid_gen = cass_uuid_gen_new();
    uv_mutex_init(&mutex);
    uv_cond_init(&cond);
    connect_session(session, cluster, on_session_connect);
    /* Code running in parallel with queries */
    wait_exit();
    uv_cond_destroy(&cond);
    uv_mutex_destroy(&mutex);
    cass_cluster_free(cluster);
    cass_uuid_gen_free(uuid_gen);
    cass_session_free(session);
    return 0;
}

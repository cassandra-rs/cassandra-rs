extern crate cassandra;
extern crate cassandra_sys;
use cassandra::*;
use cassandra_sys::cass_cluster_new;
use cassandra_sys::cass_session_new;
use cassandra_sys::cass_session_connect;
use cassandra_sys::cass_future_error_code;
use cassandra_sys::CASS_OK;
use cassandra_sys::cass_cluster_set_contact_points;
use std::ffi::CString;
use std::str::FromStr;

static QUERY: &'static str = "SELECT keyspace_name FROM system_schema.keyspaces;";
static COL_NAME: &'static str = "keyspace_name";

fn main() {unsafe{
    let mut cluster = Cluster::new();
  	cluster.set_contact_points(ContactPoints::from_str("127.0.0.1").unwrap()).unwrap();

//  	let session = cass_session_new();
  	let session = cluster.connect().unwrap();

    cluster.set_contact_points(ContactPoints::from_str("127.0.0.1").unwrap()).unwrap();
    cluster.set_load_balance_round_robin().unwrap();
    let session = cluster.connect().unwrap();
	    let result = session.execute(QUERY, 0).wait().unwrap();
    	println!("{}", result);
    	for row in result.iter() {
        println!("ks name = {}", row.get_column_by_name(COL_NAME));
    //session.close().wait().unwrap();
}}

}
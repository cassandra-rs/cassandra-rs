use cassandra_cpp::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!(
            "Usage: {} <secure connect bundle zip> <username> <password>",
            args[0]
        );
        return;
    }

    let secure_connection_bundle = &args[1];
    let username = &args[2];
    let password = &args[3];

    let mut cluster = Cluster::default();
    cluster
        .set_cloud_secure_connection_bundle(secure_connection_bundle)
        .unwrap();
    cluster.set_credentials(username, password).unwrap();

    let session = cluster.connect().unwrap();
    let statement = stmt!("SELECT release_version FROM system.local");
    let result = session.execute(&statement).wait().unwrap();
    let row = result.first_row().unwrap();
    let version: String = row.get_by_name("release_version").unwrap();
    println!("release_version: {}", version);
}

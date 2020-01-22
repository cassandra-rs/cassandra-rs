use cassandra_cpp::*;
use std::fs;

fn main() {
    let query = stmt!("SELECT keyspace_name FROM system_schema.keyspaces;");
    let col_name = "keyspace_name";
    let contact_points = "127.0.0.1";
    let tls_ca_certificate_path = "ca/certificate/path";

    let ca_cert = match fs::read_to_string(tls_ca_certificate_path) {
        Ok(cert) => {
            println!("Read in certificate file");
            cert
        }
        Err(e) => {
            panic!("Failed to open certificate file. Error: {}", e);
        }
    };

    let mut ssl = cassandra_cpp::Ssl::default();

    match cassandra_cpp::Ssl::add_trusted_cert(&mut ssl, &ca_cert) {
        Ok(_o) => {
            println!("Added trusted certificate");
        }
        Err(e) => {
            panic!("Failed to add trusted certificate. Error: {}", e);
        }
    }

    println!("Set verification level");
    let verify_level = vec![cassandra_cpp::SslVerifyFlag::PEER_IDENTITY];
    cassandra_cpp::Ssl::set_verify_flags(&mut ssl, &verify_level);

    let mut cluster = Cluster::default();
    cluster.set_contact_points(contact_points).unwrap();
    cluster.set_load_balance_round_robin();

    println!("Adding SSL to cluster");
    cluster.set_ssl(&mut ssl);

    match cluster.connect() {
        Ok(ref mut session) => {
            let result = session.execute(&query).wait().unwrap();
            println!("{}", result);
            for row in result.iter() {
                let col: String = row.get_by_name(col_name).unwrap();
                println!("ks name = {}", col);
            }
        }
        err => println!("{:?}", err),
    }
}

extern crate cassandra_cpp;
extern crate futures;

use cassandra_cpp::*;
use futures::Future;
use std::fs;

fn main() {
    let query = stmt!("SELECT keyspace_name FROM system_schema.keyspaces;");
    let col_name = "keyspace_name";

    let contact_points = "127.0.0.1";
    let tls_ca_certificate_path = Some("ca/certificate/path");

    let ca_cert = match &tls_ca_certificate_path.clone() {
        Some(path) => match fs::read_to_string(path) {
            Ok(cert) => {
                println!("Read in certificate file");
                Some(cert)
            }
            Err(e) => {
                panic!("Failed to open certificate file. Error: {}", e);
            }
        },
        None => None,
    };

    let mut sslopt: Option<cassandra_cpp::Ssl> = None;
    if let Some(cert) = ca_cert.clone() {
        let mut ssl = cassandra_cpp::Ssl::default();
        match cassandra_cpp::Ssl::add_trusted_cert(&mut ssl, &cert) {
            Ok(_o) => {
                println!("Added trusted certificate");
            }
            Err(e) => {
                panic!("Failed to add trusted certificate. Error: {}", e);
            }
        }
        sslopt = Some(ssl);
    };

    if let Some(ref mut ssl) = sslopt {
        println!("Set verification level");
        let verify_level = vec![cassandra_cpp::SslVerifyFlag::PEER_IDENTITY];
            cassandra_cpp::Ssl::set_verify_flags(ssl, &verify_level);
    }

    let mut cluster = Cluster::default();
    cluster.set_contact_points(contact_points).unwrap();
    cluster.set_load_balance_round_robin();

    if let Some(ref mut ssl) = sslopt {
                                    println!("Adding SSL to cluster");
                                    cluster.set_ssl(ssl);
                                }

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


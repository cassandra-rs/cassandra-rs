//! Simple example demonstrating the use of set/map/list values in both
//! bindings and results.
use cassandra_cpp::*;
use std::collections::hash_map::HashMap;

fn do_work(session: &Session) -> Result<()> {
    let create_keyspace = stmt!("CREATE KEYSPACE IF NOT EXISTS testks WITH REPLICATION = { 'class': 'SimpleStrategy', 'replication_factor': 1 };");
    let create_table = stmt!("CREATE TABLE IF NOT EXISTS testks.user (first_name text PRIMARY KEY, addresses map<text, text>, email set<text>, last_name text, phone_numbers list<text>, title int);");
    let mut insert_data = stmt!("INSERT INTO testks.user (first_name, addresses, email, last_name, phone_numbers, title) VALUES (?, ?, ?, ?, ?, ?);");
    let query = stmt!("SELECT * FROM testks.user;");

    session.execute(&create_keyspace).wait()?;

    session.execute(&create_table).wait()?;

    insert_data.bind(0, "Paul")?;
    insert_data.bind_null(1)?;
    let mut addresses = List::new(0);
    addresses.append_string("george@example.com")?;
    addresses.append_string("paul@example.com")?;
    insert_data.bind(2, addresses)?;
    insert_data.bind(3, "George")?;
    let mut phones = List::new(0);
    phones.append_string("123-456")?;
    phones.append_string("789-012")?;
    insert_data.bind(4, phones)?; // TODO: bind should really accept Vec<T>, and map should accept HashMap<T, U>. Requires generic CassCollection::append.
    insert_data.bind(5, 13)?;
    session.execute(&insert_data).wait()?;

    let result = session.execute(&query).wait()?;

    println!("Overall result: {}", result);
    for row in result.iter() {
        println!("Row: {}", row);

        let first_name: String = row.get_by_name("first_name")?;
        let addresses: HashMap<String, String> = {
            let maybe_iter: Result<MapIterator> = row.get_by_name("addresses");
            match maybe_iter {
                Err(_) => HashMap::new(),
                Ok(addresses_iter) => addresses_iter
                    .map(|(k, v)| Ok((k.get_string()?, v.get_string()?)))
                    .collect::<Result<_>>()?,
            }
        };
        let emails_iter: SetIterator = row.get_by_name("email")?;
        let emails: Vec<String> = emails_iter
            .map(|v| Ok(v.get_string()?))
            .collect::<Result<_>>()?;
        let last_name: String = row.get_by_name("last_name")?;
        let phone_numbers_iter: SetIterator = row.get_by_name("phone_numbers")?;
        let phone_numbers: Vec<String> = phone_numbers_iter
            .map(|v| Ok(v.get_string()?))
            .collect::<Result<_>>()?;
        let title: i32 = row.get_by_name("title")?;
        println!(
            " == {} {:?} {:?} {} {:?} {}",
            first_name, addresses, emails, last_name, phone_numbers, title
        );
    }
    Ok(())
}

fn main() {
    let contact_points = "127.0.0.1";
    let mut cluster = Cluster::default();
    cluster.set_contact_points(contact_points).unwrap();
    cluster.set_load_balance_round_robin();

    let session = cluster.connect().unwrap();
    do_work(&session).unwrap();
}

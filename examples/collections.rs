//! Simple example demonstrating the use of set/map/list values in both
//! bindings and results.
use cassandra_cpp::*;
use std::collections::hash_map::HashMap;

async fn do_work(session: &Session) -> Result<()> {
    session.execute("CREATE KEYSPACE IF NOT EXISTS testks WITH REPLICATION = { 'class': 'SimpleStrategy', 'replication_factor': 1 };").await?;
    session.execute("CREATE TABLE IF NOT EXISTS testks.user (first_name text PRIMARY KEY, addresses map<text, text>, email set<text>, last_name text, phone_numbers list<text>, title int);").await?;

    let mut insert_data = session.statement("INSERT INTO testks.user (first_name, addresses, email, last_name, phone_numbers, title) VALUES (?, ?, ?, ?, ?, ?);");

    insert_data.bind(0, "Paul")?;
    insert_data.bind_null(1)?;
    let mut addresses = List::new();
    addresses.append_string("george@example.com")?;
    addresses.append_string("paul@example.com")?;
    insert_data.bind(2, addresses)?;
    insert_data.bind(3, "George")?;
    let mut phones = List::new();
    phones.append_string("123-456")?;
    phones.append_string("789-012")?;
    insert_data.bind(4, phones)?; // TODO: bind should really accept Vec<T>, and map should accept HashMap<T, U>. Requires generic CassCollection::append.
    insert_data.bind(5, 13)?;
    insert_data.execute().await?;

    let result = session.execute("SELECT * FROM testks.user;").await?;

    println!("Overall result: {}", result);
    let mut iter = result.iter();
    while let Some(row) = iter.next() {
        println!("Row: {}", row);

        let first_name: String = row.get_by_name("first_name")?;
        let addresses: HashMap<String, String> = {
            let maybe_iter: Result<MapIterator> = row.get_by_name("addresses");
            match maybe_iter {
                Err(_) => HashMap::new(),
                Ok(mut addresses_iter) => {
                    let mut map = HashMap::new();
                    while let Some((k, v)) = addresses_iter.next() {
                        map.insert(k.get_string()?, v.get_string()?);
                    }
                    map
                }
            }
        };
        let mut emails_iter: SetIterator = row.get_by_name("email")?;
        let mut emails: Vec<String> = vec![];
        while let Some(v) = emails_iter.next() {
            emails.push(v.get_string()?);
        }
        let last_name: String = row.get_by_name("last_name")?;
        let mut phone_numbers_iter: SetIterator = row.get_by_name("phone_numbers")?;
        let mut phone_numbers: Vec<String> = vec![];
        while let Some(v) = phone_numbers_iter.next() {
            phone_numbers.push(v.get_string()?);
        }
        let title: i32 = row.get_by_name("title")?;
        println!(
            " == {} {:?} {:?} {} {:?} {}",
            first_name, addresses, emails, last_name, phone_numbers, title
        );
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut cluster = Cluster::default();
    cluster.set_contact_points("127.0.0.1").unwrap();
    cluster.set_load_balance_round_robin();

    let session = cluster.connect().await?;
    do_work(&session).await
}

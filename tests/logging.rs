#[macro_use(stmt)]
extern crate cassandra_cpp;
extern crate slog;

mod help;

use cassandra_cpp::*;

use std::sync::Arc;
use std::sync::Mutex;
use slog::*;

#[derive(Clone)]
struct MyDrain(Arc<Mutex<String>>);

impl Default for MyDrain {
    fn default() -> Self {
        MyDrain(Arc::new(Mutex::new("".to_string())))
    }
}

impl Drain for MyDrain {
    type Ok = ();
    type Err = ();

    fn log(&self, record: &Record, values: &OwnedKVList) -> ::std::result::Result<Self::Ok, Self::Err> {
        self.0.lock().unwrap().push_str(&format!("{}", record.msg()));
        Ok(())
    }
}

#[test]
fn test_logging() {
    let drain = MyDrain::default();
    let logger = Logger::root(drain.clone().fuse(), o!());

    set_level(LogLevel::WARN);
    set_logger(logger);

    let mut cluster = Cluster::default();
    cluster.set_contact_points("absolute-gibberish.invalid").unwrap();
    cluster.connect().expect_err("Should fail to connect");

    assert!(drain.0.lock().unwrap().contains("cannot for hte life of me"), drain.0);
}

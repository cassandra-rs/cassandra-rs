#[macro_use(stmt)]
extern crate cassandra_cpp;

mod help;

use cassandra_cpp::*;

#[test]
fn test_using_consistency() {
    let mut s = stmt!("select 1+1;");
    s.set_consistency(Consistency::LOCAL_ONE).unwrap();
    s.set_serial_consistency(Consistency::SERIAL).unwrap();

    let mut batch = Batch::new(CASS_BATCH_TYPE_LOGGED);
    batch.add_statement(&s).unwrap();
    batch.set_consistency(Consistency::TWO).unwrap();
    batch.set_serial_consistency(Consistency::LOCAL_SERIAL).unwrap();
}

#[test]
fn test_parsing_printing_consistency() {
    let all = Consistency::variants();

    for c in all {
        let s = c.to_string();
        let c2: Consistency = s.parse().expect(&format!("Failed on {:?} as {}", c, s));
        assert_eq!(c2, *c, "with intermediate {}", s);
    }

    // Just a few spot checks to confirm the formatting hasn't regressed
    // or changed unexpectedly.
    assert_eq!(Consistency::LOCAL_QUORUM.to_string(), "LOCAL_QUORUM");
    assert_eq!(format!("{}", Consistency::LOCAL_QUORUM), "LOCAL_QUORUM");
    assert_eq!("THREE".parse::<Consistency>().unwrap(), Consistency::THREE);
    let _ = "INVALID".parse::<Consistency>().expect_err("Should have failed to parse");
}

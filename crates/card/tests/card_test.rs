use cqrs_es::test::TestFramework;

use card::aggregate::CardAggregate;

type CardTestFramework = TestFramework<CardAggregate>;

// todo: create fixtures for testing

#[test]
fn test_something() {
    assert_eq!(1, 1);
}

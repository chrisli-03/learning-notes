use testing::{sploosh, splish};

#[test]
fn test_c() {
    assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
}
use let_match::let_match;

#[test]
fn let_match_ident() {
    let_match!(x, 1);
    assert_eq!(x, 1);
}

#[test]
fn let_match_tuple() {
    let_match!((x,), (1,));
    assert_eq!(x, 1);

    let_match!((x, y), (1, 2));
    assert_eq!(x, 1);
    assert_eq!(y, 2);
}

#[test]
fn let_match_struct() {
    struct TestStruct {
        x: u8,
        y: &'static str,
    }

    let_match!(TestStruct { x, y }, TestStruct { x: 1, y: "hi" });
    assert_eq!(x, 1);
    assert_eq!(y, "hi");

    let_match!(TestStruct { x: y, y: x }, TestStruct { x: 1, y: "hi" });
    assert_eq!(x, "hi");
    assert_eq!(y, 1);

    let_match!(TestStruct { x: a, .. }, TestStruct { x: 1, y: "hi" });
    assert_eq!(a, 1);
}

#[test]
fn let_match_tuplestruct() {
    struct TestTupleStruct(u8, bool);

    let_match!(TestTupleStruct(x, y), TestTupleStruct(4, true));
    assert_eq!(x, 4);
    assert_eq!(y, true);
}

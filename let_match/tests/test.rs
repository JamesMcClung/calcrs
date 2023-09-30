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

#[test]
fn let_match_enum() {
    enum TestEnum {
        TupleEnum1(u8),
        TupleEnum2(u8, bool),
        StructEnum { x: i8, y: char },
    }

    let_match!(TestEnum::TupleEnum1(x), TestEnum::TupleEnum1(4));
    assert_eq!(x, 4);

    let_match!(TestEnum::TupleEnum2(x, y), TestEnum::TupleEnum2(3, false));
    assert_eq!(x, 3);
    assert_eq!(y, false);

    let_match!(TestEnum::StructEnum { x, y }, TestEnum::StructEnum { x: -3, y: 'a' });
    assert_eq!(x, -3);
    assert_eq!(y, 'a');

    let_match!(TestEnum::StructEnum { y: x, x: y }, TestEnum::StructEnum { x: -3, y: 'a' });
    assert_eq!(x, 'a');
    assert_eq!(y, -3);
}

#[test]
#[should_panic]
fn let_match_wrong_variant1() {
    let_match!(Some(_x), Option::<u8>::None);
}

#[test]
#[should_panic]
fn let_match_wrong_variant2() {
    let_match!(Ok(_x), Result::<u8, u8>::Err(3));
}

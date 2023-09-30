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

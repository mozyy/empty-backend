use empty_macro::add_field;

#[test]
fn test_macro() {
    #[add_field]
    struct Pancakes {}
    let _a = Pancakes {
        a: String::from("_"),
    };
}

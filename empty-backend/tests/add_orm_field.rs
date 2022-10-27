use crate::add_orm_field;

#[test]
fn test_macro() {
    #[add_orm_field]
    struct Pancakes {}
}

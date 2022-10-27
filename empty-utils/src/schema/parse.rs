use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Ident, Token,
};
#[derive(Default)]
pub struct Attrs {
    pub table_name: Option<Ident>,
}

//table_name = questions
impl Parse for Attrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let table_name: Ident = input.parse()?;
        if table_name != "table_name" {
            return Err(Error::new(table_name.span(), "需要table_name"));
        }
        let _token: Token![=] = input.parse()?;
        let name: Ident = input.parse()?;
        Ok(Attrs {
            table_name: Some(name),
        })
    }
}

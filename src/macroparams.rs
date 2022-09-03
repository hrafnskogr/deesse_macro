use syn::parse::{Parse, ParseStream};
use syn::{Ident, Token, LitStr};
use syn::punctuated::Punctuated;
use syn::Result;

#[derive(Debug)]
pub struct MacroParams
{
    pub syscall: LitStr,
    pub args: Punctuated<Ident, Token![,]>,
}


impl Parse for MacroParams
{
    fn parse(input: ParseStream) -> Result<Self>
    {
        let sysc: LitStr = input.parse()?;

        let colon: Option<Token![,]> = match input.parse()
        {
            Ok(token) => Some(token),
            _ => None,
        };

        let args: Punctuated<Ident, Token![,]> = input.parse_terminated(Ident::parse)?;

        Ok( MacroParams
            {
                syscall: sysc,
                args: args,
            })
    }        
}

extern crate proc_macro;

mod macroparams;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use sha256::digest;
use deesse;

use macroparams::MacroParams;


#[proc_macro]
pub fn ds(input: TokenStream) -> TokenStream
{
    let params = syn::parse_macro_input!(input as MacroParams);
    let args_number = params.args.len();
    let params_args = params.args;

    println!("{}", args_number); 
   
    let nargs = if args_number == 0
    {
        String::from("")
    }
    else
    {
        args_number.to_string()
    };
    
    let sysc = format_ident!("syscall{}", nargs);

    let args = if args_number == 0
    {
        quote!{}
    }
    else
    {
        quote!{, #params_args }
    };


    let salt_call = format!("{}{}", deesse::SALT, params.syscall.value());
    let h_call = digest(salt_call);

    TokenStream::from(
        quote! {
            deesse::#sysc(deesse::NT.get().get_syscall(#h_call) #args);
        })
}


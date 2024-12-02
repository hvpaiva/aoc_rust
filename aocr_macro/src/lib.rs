use aocr::runner::Part;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse, parse_macro_input, Ident, ItemFn, LitStr, Token};

struct AocArgs {
    part: Option<LitStr>,
    name: Option<LitStr>,
}

impl From<AocArgs> for Aoc {
    fn from(args: AocArgs) -> Self {
        let part = args
            .part
            .map_or_else(|| "one".to_string(), |lit_str| lit_str.value())
            .into();

        let name = args
            .name
            .map_or_else(|| "solution".to_string(), |lit_str| lit_str.value());

        Aoc { part, name }
    }
}

struct Aoc {
    part: Part,
    name: String,
}

impl Parse for AocArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut part: Option<LitStr> = None;
        let mut name: Option<LitStr> = None;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;

            input.parse::<Token![=]>()?;

            if ident == "part" {
                part = Some(input.parse()?);
            } else if ident == "name" {
                name = Some(input.parse()?);
            } else {
                return Err(syn::Error::new(ident.span(), "Unknown argument for #[aoc]"));
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(AocArgs { part, name })
    }
}

#[proc_macro_attribute]
pub fn aoc(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AocArgs);
    let input_fn = parse_macro_input!(input as ItemFn);

    let args: Aoc = args.into();

    let fn_name = &input_fn.sig.ident;
    let (part, name) = (args.part.as_str(), args.name);

    let register_fn_name = format_ident!("register_function_{}_{}", part, name);

    let gen = quote! {
        #input_fn

        #[ctor::ctor]
        fn #register_fn_name() {
            aocr::runner::register_function(#part, #name, #fn_name);
        }
    };

    TokenStream::from(gen)
}

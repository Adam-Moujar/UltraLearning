use proc_macro::TokenStream;
use quote::quote;

// This is the macro derivation
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Tokenstream is a stream of tokens(duh) its, what the code injection
    // looks like, and what the output will be.
    // if the syn parser fails we need to panic
    // unrecoverable error, in actualy production code
    // we should panic! with a specific name rather than just unwrap
    // but for now, unwrap is good enough
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // We get the name of the struct we are deriving
    let name = &ast.ident;
    // This is the generated code
    let generated = quote! {
        // impl HelloMacro for the struct
        impl HelloMacro for #name{
            fn hello_macro(){
                // stringify takes an expression, e.g 1 + 2 and turns it into a string -> "1 + 2"
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}

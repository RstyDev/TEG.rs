use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenStream as TokenStream2, TokenTree, Delimiter};
use quote::quote;


#[proc_macro]
pub fn arc_mutex(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    match input.into_iter().next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            let inner_stream = group.stream();
            quote! {
                Arc::new(Mutex::new(#inner_stream))
            }.into()
        }
        _ => panic!("Expected a group of tokens enclosed in parentheses for arc_mutex macro"),
    }
}

#[proc_macro]
pub fn string(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    match input.into_iter().next() {
        Some(TokenTree::Literal(lit)) => {
            println!("Literal: {:#?}", lit);
            let literal = lit.to_string().trim_matches('"').to_string();
            quote! {
                String::from(#literal)
            }.into()
        }
        None => quote! {
            String::new()
        }.into(),
        _ => panic!("Expected a string literal for string macro"),
    }
}

#[proc_macro]
pub fn hashmap(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    // let map = std::collections::HashMap::new();
    let elements = input.into_iter().filter_map(|token| {
        if let TokenTree::Group(group) = token {
            if group.delimiter() != Delimiter::Brace {
                panic!("Expected a group of tokens enclosed in braces for hashmap macro");
            }
            
            let stream = group.stream();
            let mut key = TokenTree::Literal(Literal::string(""));
            let mut value = TokenTree::Literal(Literal::string(""));

            for (i,inner_token) in stream.into_iter().enumerate() {
                match inner_token {
                    TokenTree::Ident(ident) if i % 3 == 0 => {
                        key = TokenTree::Ident(ident);
                    }
                    TokenTree::Literal(lit) if i % 3 == 0 => {
                        key = TokenTree::Literal(lit);
                    }
                    TokenTree::Punct(a) if i % 3 == 1 && a.as_char() == ':' => {
                        continue
                    }
                    TokenTree::Ident(ident) if i % 3 == 2 => {
                        value = TokenTree::Ident(ident);
                    }
                    TokenTree::Literal(lit) if i % 3 == 2 => {
                        value = TokenTree::Literal(lit);
                    }
                    _ => panic!("Unexpected token in hashmap macro"),
                }
            }

            Some(quote! {
                (#key, #value)
            })
        } else {
            None
        }
    }).collect::<Vec<_>>();

    quote! {
        {
            let mut _map: std::collections::HashMap<_, _> = [#(#elements),*].into_iter().collect();
            _map
        }
    }.into()

}
use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, ExprAssign};
use quote::{quote, ToTokens};

// should place in the given value into an array of two elements and the second element should be the given value multiplied by two
// like this: [value, value * 2]
// but it should also be called like this and sub in all values in a given range
// data_range!(1..100) => [[1, 2], [2, 4], [3, 6], ...]
#[proc_macro]
pub fn data_range(input: TokenStream) -> TokenStream {
    struct DataRange(Vec<[i32; 2]>);
    impl ToTokens for DataRange {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            let inner_tokens = self.0.iter().map(|arr| {
                let (value1, value2) = (arr[0], arr[1]);
                quote! { [#value1, #value2], }
            });
            tokens.extend(inner_tokens);
        }
        fn to_token_stream(&self) -> proc_macro2::TokenStream {
            let mut tokens = proc_macro2::TokenStream::new();
            self.to_tokens(&mut tokens);
            tokens
        }
        fn into_token_stream(self) -> proc_macro2::TokenStream
            where Self: Sized
        {
            self.to_token_stream()
        }
    }
    let input = parse_macro_input!(input as Expr);
    let expanded = match input {
        syn::Expr::Range(range) => {
            let (start, end) = (
                range.start.expect("Expected a start value"), 
                range.end.expect("Expected an end value")
            );
            let start = if let Expr::Lit(start) = *start {
                let start = start.into_token_stream().to_string();
                println!("start: {}", start);
                start.parse().expect("Expected a number value")
            } else {
                panic!("Expected a number value");
            };
            let end = if let Expr::Lit(end) = *end {
                let end = end.into_token_stream().to_string();
                println!("end: {}", end);
                end.parse::<i32>().expect("Expected a number value") + 1
            } else {
                panic!("Expected a number value");
            };

            let mut output: Vec<[i32; 2]> = Vec::new();
            for i in start..end {
                output.push([i, i * 2]);
            }
            let output = DataRange(output);
            let expanded = quote! { &[ #output ] };

            expanded
        },
        _ => panic!("Expected a range expression")
    };
    /*let (start, end) = (0, 0);
    let (start, end): (i32, i32) = (
        start.to_string().parse().expect("Expected a number value"), 
        end.to_string().parse().expect("Expected a number value")
    ); let end = end + 1;
    let mut output: Vec<[i32; 2]> = Vec::new();
    for i in start..end {
        output.push([i, i * 2]);
    }
    let output = DataRange(output);
    let expanded = quote! { &[ #output ] };*/

    TokenStream::from(expanded)
}

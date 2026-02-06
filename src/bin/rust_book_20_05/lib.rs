// use proc_macro::{TokenStream};

#[macro_export]
macro_rules! vect {
    ( $( $x:expr ),* ) => {
        {
            let mut tmp_vec = Vec::new();
            $(
                tmp_vec.push($x);
            )*
            tmp_vec
        }
    };
}

// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {
//
// }


pub trait HelloMacro {
    fn hello_macro();
}

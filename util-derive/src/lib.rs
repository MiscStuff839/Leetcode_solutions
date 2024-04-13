macro_rules! sol {
    (fn($( $arg:ident: $type:ty ),*) -> $ret:ty) => {
        struct Solution;

        impl Solution {
            fn $crate($($arg: $type,)*) -> $ret {
                
            }
        }
    };
}
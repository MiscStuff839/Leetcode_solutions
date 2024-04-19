use std::{fs, path::PathBuf, str::FromStr};

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn generate_list(_: TokenStream) -> TokenStream {
    let mut dir = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    dir.pop();
    dir.push("leetcode");
    dir.push("src");
    dir.push("solutions");
    dbg!(dir.clone());
    let members: Vec<String> = fs::read_dir(dir)
        .expect("solutions directory does not exist")
        .filter_map(|p| {
            let name = p.expect("Entry does not exist").file_name();
            if name != "mod.rs" {
                Some(name.to_str().expect("unable to convert")[0..name.len()-3].to_string())
            } else {
                None
            }
        })
        .collect();
    let array = quote! {
        [
            #(#members),*
        ]
    };
    let len = members.len();
    let output: proc_macro2::TokenStream = quote! {
        pub const MODS: [&str; #len] = #array;
    };
    output.into()
}

// #[proc_macro_attribute]
// pub fn solt(_: TokenStream, module: TokenStream) -> TokenStream {
//     let ast: syn::ItemMod = syn::parse(module).unwrap();
//     assert!(ast.semi.is_none());
//     fn collect_names(vec: &mut Vec<Ident>, f: &ItemFn) {
//         vec.push(f.sig.ident.clone());
//     }
//     let mut idents = vec![];
//     let content = ast.content.unwrap().1.clone().iter().filter_map(|i| {
//         if let Item::Fn(f) = i {
//             collect_names(&mut idents, f);
//             Some(f.clone().block.into_token_stream())
//         } else {
//             None
//         }
//     }).collect::<Vec<proc_macro2::TokenStream>>();
//     let content = quote! {#(#content)*};
//     quote! {
//         pub fn run_tests() {
//             #content
//         }
//     }.into()
// }
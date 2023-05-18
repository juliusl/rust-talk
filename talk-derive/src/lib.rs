use syn::parse_macro_input;
use quote::*;

mod store_trait;
use store_trait::StoreTrait;

#[proc_macro_derive(Store, attributes(storage))]
pub fn derive_store(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let store_trait = parse_macro_input!(input as StoreTrait);

    let read_trait = store_trait.read_trait();
    let create_trait = store_trait.create_trait();
    let delete_trait = store_trait.delete_trait();
    let replace_trait = store_trait.replace_trait();
    let store_trait = store_trait.store_trait();

    quote! {
        #read_trait
        #create_trait
        #delete_trait
        #replace_trait
        #store_trait
    }.into()
}

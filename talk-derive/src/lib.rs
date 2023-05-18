use syn::{parse_macro_input, DeriveInput};
use quote::*;
// use proc_macro2::*;

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

// Example It 1
// #[proc_macro_derive(Store)]
// pub fn derive_store(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let derive_input = parse_macro_input!(input as DeriveInput);

//     let ty_name = &derive_input.ident;
//     let (impl_generics, ty_generics, where_clause) =  derive_input.generics.split_for_impl();

//     quote! {
//         impl #impl_generics Read #ty_generics for #ty_name #ty_generics #where_clause
//         {
//             fn read(&self, item: &T) -> Result<&T, Error> {
//                 if let Some((_, t)) = self.find(item) {
//                     Ok(t)
//                 } else {
//                     Err(Error::new())
//                 }
//             }
//         }
        
//         impl #impl_generics Create #ty_generics for #ty_name #ty_generics #where_clause
//         {
//             fn create(&mut self, item: &T) -> Result<&T, Error> {
//                 let cloned = item.clone();
//                 self.push(cloned);
//                 Ok(self.last().expect("should exist just added"))
//             }
//         }
        
//         impl #impl_generics Delete #ty_generics for #ty_name #ty_generics #where_clause
//         {
//             fn delete(&mut self, item: &T) -> Result<T, Error> {
//                 if let Some((loc, _)) = self.find(item) {
//                     Ok(self.remove_by_index(loc).expect("should exist just found it"))
//                 } else {
//                     Err(Error::new())
//                 }
//             }
//         }
        
//         impl #impl_generics Replace #ty_generics for #ty_name #ty_generics #where_clause
//         {
//             fn replace(&mut self, item: &T) -> Result<&T, Error> {
//                 if let Some((loc, _item)) = self.find_mut(item) {
//                     *_item = item.clone();
//                     Ok(self.get(loc).expect("should exist jsut added"))
//                 } else {
//                     Err(Error::new())
//                 }
//             }
//         }
//     }.into()
// }
use proc_macro2::*;
use quote::quote;
use syn::{parse::Parse, Generics, DeriveInput};

pub struct StoreTrait {
    ident: Ident,
    generics: Generics,
    storage_field: Option<Ident>,
}

impl Parse for StoreTrait {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_input = DeriveInput::parse(input)?;

        let storage = match derive_input.data {
            syn::Data::Struct(d) => {
                d.fields.iter().filter_map(|f| {
                    for a in &f.attrs {
                        if a.path().is_ident("storage") {
                            return f.ident.clone();
                        }
                    }

                    None
                }).next()
            },
            _ => {
                return Err(input.error("Missing a storage attribute"));
            }
        };
        
        Ok(Self {
            ident: derive_input.ident,
            generics: derive_input.generics,
            storage_field: storage
        })
    }
}

impl StoreTrait {

    /// Returns an expressions for the storage receiver,
    /// 
    pub fn storage_receiver(&self) -> TokenStream {
        if let Some(storage) = self.storage_field.as_ref() {
            quote! {
                self.#storage
            }
        } else {
            quote! {
                self
            }
        }
    }

    /// Returns a store trait,
    /// 
    pub fn store_trait(&self) -> TokenStream {
        let ty_name = &self.ident;
        let (impl_generics, ty_generics, where_clause) =  self.generics.split_for_impl();

        quote! {
            impl #impl_generics Store #impl_generics for #ty_name #ty_generics #where_clause
                Self: From<StoreInfo> + AsRef<StoreInfo>,
            {
            }
        }
    }

    /// Returns a create trait impl,
    /// 
    pub fn create_trait(&self) -> TokenStream {
        let ty_name = &self.ident;
        let mut param = self.generics.type_params();
        let param = param.next().expect("should have a param");
        let param = &param.ident;

        let (impl_generics, ty_generics, where_clause) =  self.generics.split_for_impl();
        
        let storage_receiver = self.storage_receiver();

        quote! {
            impl #impl_generics Create #ty_generics for #ty_name #ty_generics #where_clause
            {
                fn create(&mut self, item: &#param) -> Result<&T, Error> {
                    #storage_receiver.insert_item(item)?;
                    self.read(item)
                }
            }
        }
    }

    pub fn read_trait(&self) -> TokenStream {
        let ty_name = &self.ident;
        let mut param = self.generics.type_params();
        let param = param.next().expect("should have a param");
        let param = &param.ident;

        let (impl_generics, ty_generics, where_clause) =  self.generics.split_for_impl();

        let storage_receiver = self.storage_receiver();

        quote! {
            impl #impl_generics Read #ty_generics for #ty_name #ty_generics #where_clause
            {
                fn read(&self, item: &#param) -> Result<&T, Error> {
                    if let Some((_, t)) = #storage_receiver.find(item) {
                        Ok(t)
                    } else {
                        Err(Error::new())
                    }
                }
            }
        }
    }

    pub fn replace_trait(&self) -> TokenStream {
        let ty_name = &self.ident;
        let mut param = self.generics.type_params();
        let param = param.next().expect("should have a param");
        let param = &param.ident;

        let (impl_generics, ty_generics, where_clause) =  self.generics.split_for_impl();

        let storage_receiver = self.storage_receiver();

        quote! {
            impl #impl_generics Replace #ty_generics for #ty_name #ty_generics #where_clause
            {
                fn replace(&mut self, item: &#param) -> Result<&T, Error> {
                    if let Some((loc, _item)) = #storage_receiver.find_mut(item) {
                        *_item = item.clone();
                        self.read(item)
                    } else {
                        Err(Error::new())
                    }
                }
            }
        }
    }

    pub fn delete_trait(&self) -> TokenStream {
        let ty_name = &self.ident;
        let mut param = self.generics.type_params();
        let param = param.next().expect("should have a param");
        let param = &param.ident;

        let (impl_generics, ty_generics, where_clause) =  self.generics.split_for_impl();

        let storage_receiver = self.storage_receiver();

        quote! {
            impl #impl_generics Delete #ty_generics for #ty_name #ty_generics #where_clause
            {
                fn delete(&mut self, item: &#param) -> Result<T, Error> {
                    if let Some((loc, _)) = #storage_receiver.find(item) {
                        Ok(#storage_receiver.remove_by_index(loc).expect("should exist just found it"))
                    } else {
                        Err(Error::new())
                    }
                }
            }
        }
    }
}
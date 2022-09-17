use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(DiskStruct)]
pub fn disk_struct_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap(); // will panic is parsing fails
    impl_disk_struct(&ast)
}

fn impl_disk_struct(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    match data {
        syn::Data::Struct(s) => {
            let fields = &s.fields;

            // growable quotes for each trait function.
            let mut new_quote = quote!();
            let mut to_bytes_quote = quote!();
            let mut update_from_bytes_quote = quote!();
            let mut length_quote = quote!();

            match fields {
                syn::Fields::Named(fields_named) => {
                    for field_node in fields_named.named.iter() {
                        let field_name = field_node.ident.as_ref().unwrap();
                        let field_type = &field_node.ty;
                        match field_type {
                            syn::Type::Array(t) => {
                                let array_len = &t.len;
                                new_quote.extend(quote! {
                                    #field_name: [0;#array_len],
                                });
                                to_bytes_quote.extend(quote! {
                                    ans.append(&mut self.#field_name.to_vec());
                                });
                                update_from_bytes_quote.extend(quote! {
                                    for i in 0..self.#field_name.len() {
                                        self.#field_name[i] = dat[offset+i];
                                    }
                                    offset += self.#field_name.len();
                                });
                                length_quote.extend(quote! {
                                    ans += self.#field_name.len();
                                });
                            }
                            syn::Type::Path(t) => {
                                let mut ident = String::from("");
                                for seg in t.path.segments.iter() {
                                    ident = seg.ident.to_string();
                                }
                                if ident!="u8" {
                                    panic!("derivation of DiskStruct only possible for u8,[u8]");
                                }
                                new_quote.extend(quote! {
                                    #field_name: 0,
                                });
                                to_bytes_quote.extend(quote! {
                                    ans.push(self.#field_name);
                                });
                                update_from_bytes_quote.extend(quote! {
                                    self.#field_name = dat[offset];
                                    offset += 1;
                                });
                                length_quote.extend(quote! {
                                    ans += 1;
                                });
                            }
                            _ => {
                                panic!("derivation of DiskStruct only possible for u8,[u8]");
                            }
                        }
                    }
                }
                _ => unimplemented!()
            }

            let gen = quote! {
                impl DiskStruct for #name {
                    fn new() -> Self where Self: Sized {
                        Self {
                            #new_quote
                        }
                    }
                    fn to_bytes(&self) -> Vec<u8> {
                        let mut ans : Vec<u8> = Vec::new();
                        #to_bytes_quote
                        return ans;
                    }
                    fn update_from_bytes(&mut self,dat: &Vec<u8>) {
                        if dat.len()<self.len() {
                            panic!("in from_bytes, length of vector is {}, but DiskStruct is {}",dat.len(),self.len());
                        }
                        let mut offset = 0;
                        #update_from_bytes_quote

                    }
                    fn from_bytes(dat: &Vec<u8>) -> Self where Self: Sized{
                        let mut ans = Self::new();
                        ans.update_from_bytes(dat);
                        return ans;
                    }
                    fn len(&self) -> usize {
                        let mut ans = 0;
                        #length_quote
                        return ans;
                    }
                }
            };
            gen.into()
        },
        _ => panic!["derivation of DiskStruct only possible for Struct"]
    }
}

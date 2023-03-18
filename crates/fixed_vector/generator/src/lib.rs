use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;

#[proc_macro_attribute]
pub fn fixed_vector(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(attr, item).unwrap_or_else(|e| e.into_compile_error().into())
}

fn generate(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let base_struct: syn::ItemStruct = syn::parse(item)?;
    let FixedVectorArgs {
        item_type, fields, ..
    } = syn::parse(attr)?;

    let syn::ItemStruct {
        ident: identifier,
        generics,
        ..
    } = base_struct.clone();
    let field: Vec<_> = fields.iter().map(|f| quote!(#f)).collect();
    let len = field.len();

    let type_param = generics.params.iter().find_map(|arg| {
        let syn::GenericParam::Type(arg) = arg else {
            return None;
        };
        if item_type.is_ident(&arg.ident) {
            Some(arg.ident.clone())
        } else {
            None
        }
    });

    macro_rules! ops {
        ($($p: path, $m: ident, $op: tt);+) => {
            [$((quote!($p), quote!($m), quote!($op))),+]
        };
    }

    let self_binary_ops = ops!(std::ops::Add, add, +; std::ops::Sub, sub, -);

    let self_binary_op_impl = self_binary_ops.map({
        |(tr, m, op)| {
            let type_arg = type_param.as_ref().map(|t| quote!(<#t: #tr<Output=#t>>));
            let tr_assign: syn::Path = syn::parse_str(&format!("{}Assign", tr)).unwrap();
            let m_assign: syn::Ident = syn::parse_str(&format!("{}_assign", m)).unwrap();
            let op_assign: syn::BinOp = syn::parse_str(&format!("{}=", op)).unwrap();
            let type_arg_assign = type_param.as_ref().map(|t| quote!(<#t: #tr_assign>));
            quote! {
                impl #type_arg #tr for #identifier #generics {
                    type Output = Self;
                    fn #m(self, rhs: Self) -> Self {
                        Self {
                            #(#field: self.#field #op rhs.#field),*
                        }
                    }
                }

                impl #type_arg_assign #tr_assign for #identifier #generics {
                    fn #m_assign(&mut self, rhs: Self) {
                        #(self.#field #op_assign rhs.#field;)*
                    }
                }
            }
        }
    });

    let binary_ops = ops!(std::ops::Mul, mul, *;std::ops::Div, div, /);
    let binary_op_impl = binary_ops.map({
        |(tr, m, op)| {
            let type_arg = type_param
                .as_ref()
                .map(|t| quote! (<#t: Copy + #tr<Output=#t>>));
            quote! {
                impl #type_arg #tr<#item_type> for #identifier #generics {
                    type Output = Self;
                    fn #m(self, rhs: #item_type) -> Self {
                        Self {
                            #(#field: self.#field #op rhs),*
                        }
                    }
                }
            }
        }
    });

    let unary_ops = ops!(std::ops::Neg, neg, -);

    let unary_op_impl = unary_ops.map({
        |(tr, m, op)| {
            let type_arg = type_param.as_ref().map(|t| quote! (<#t: #tr<Output=#t>>));
            quote! {
                impl #type_arg #tr for #identifier #generics {
                    type Output = Self;
                    fn #m(self) -> Self {
                        Self {
                            #(#field: #op self.#field),*
                        }
                    }
                }
            }
        }
    });

    let vector_dot = {
        let type_arg = type_param.as_ref().map(|t| quote! (<#t: std::ops::Mul>));
        let where_clause = type_param
            .as_ref()
            .map(|t| quote! (where <#t as std::ops::Mul>::Output: std::iter::Sum));

        quote! {
            impl #type_arg ::fixed_vector::VectorDot<#item_type> for #identifier #generics
            #where_clause
            {
                fn dot(self, rhs: Self) -> <#item_type as std::ops::Mul>::Output {
                    [#(self.#field * rhs.#field),*].into_iter().sum()
                }
            }
        }
    };

    let others = {
        let type_arg = type_param.as_ref().map(|t| quote! (<#t>));
        quote! {
            impl #type_arg ::fixed_vector::Vector<#item_type> for #identifier #generics {
                fn len(self) -> usize {
                    #len
                }
            }
        }
    };

    let expanded = quote! {
        #base_struct

        #others

        #vector_dot

        #(#self_binary_op_impl)*

        #(#binary_op_impl)*

        #(#unary_op_impl)*
    };

    Ok(expanded.into())
}

#[derive(Debug)]
#[allow(unused)]
struct FixedVectorArgs {
    item_type: syn::Path,
    semicolon: syn::Token!(;),
    fields: syn::punctuated::Punctuated<syn::Expr, syn::Token!(,)>,
}

impl Parse for FixedVectorArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item_type = input.parse()?;
        let semicolon = input.parse()?;
        let fields = syn::punctuated::Punctuated::parse_terminated(input)?;
        Ok(Self {
            item_type,
            semicolon,
            fields,
        })
    }
}

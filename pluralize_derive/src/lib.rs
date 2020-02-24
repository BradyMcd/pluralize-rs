//

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Pluralize)]
pub fn derive_pluralize( input: TokenStream ) -> TokenStream {
    let ast = syn::parse( input ).unwrap( );

    impl_pluralize( &ast )
}

fn impl_pluralize( ast: &syn::DeriveInput ) -> TokenStream {
    let ty = &ast.ident;

    let gen = quote! {
        impl Pluralize< #ty > for #ty {
            #[inline(always)]
            fn pluralize<'a>( &'a self ) -> Iter<'a, #ty> {
                unsafe{ core::mem::transmute::<&'a #ty, &'a [#ty;1]>(self)}.iter( )
            }
            #[inline(always)]
            fn pluralize_mut<'a>( &'a mut self ) -> IterMut<'a, #ty> {
                unsafe{ core::mem::transmute::<&'a mut #ty, &'a mut [#ty;1]>(self)}.iter_mut( )
            }
        }
    };

    gen.into( )
}

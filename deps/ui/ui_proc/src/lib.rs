use std::str::FromStr;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::Parser,
    parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed, GenericParam, Ident, Type,
    __private::{Span, TokenStream2},
    spanned::Spanned,
};

#[proc_macro_attribute]
#[allow(clippy::too_many_lines)]
pub fn view(_args: TokenStream, stream: TokenStream) -> TokenStream {
    let mut stream = parse_macro_input!(stream as DeriveInput);

    let Data::Struct(data) = &mut stream.data else {
        panic!("`view` macro has to be used with structs")
    };

    let name = &stream.ident;

    let name_str =
        TokenStream2::from_str(&format!("\"{name}\"")).expect("Failed to extract view struct name");

    let generics = &stream.generics;

    let type_param_names: Vec<_> = generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(type_param) => Some(type_param.ident.clone()),
            GenericParam::Const(const_param) => Some(const_param.ident.clone()),
            GenericParam::Lifetime(_) => None,
        })
        .collect();

    let type_params = quote_spanned! {stream.generics.span()=>
        #(#type_param_names),*
    };

    let Fields::Named(fields) = &mut data.fields else {
        panic!("No named fields");
    };

    let inits = add_inits(name, fields);
    let links = add_links(fields);

    fields.named.push(
        Field::parse_named
            .parse2(quote! { view: test_engine::ui::ViewBase })
            .expect("parse2(quote! { view: test_engine::ui::ViewBase })"),
    );

    quote! {
        #[derive(derivative::Derivative, Default)]
        #[derivative(Debug)]
        #stream

        impl #generics test_engine::ui::View for #name <#type_params> {
            fn weak_view(&self) -> test_engine::refs::Weak<dyn test_engine::ui::View> {
                test_engine::refs::weak_from_ref(self as &dyn test_engine::ui::View)
            }
            fn base(&self) -> &test_engine::ui::ViewBase {
                &self.view
            }
            fn base_mut(&mut self) -> &mut test_engine::ui::ViewBase {
                &mut self.view
            }
            fn init_views(&mut self) {
                use test_engine::ui::ViewSubviews;
                #inits
            }

        }

        impl #generics test_engine::refs::AsAny for #name <#type_params> {
            fn as_any(&self) -> &dyn std::any::Any {
               self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
               self
            }
        }

        impl #generics test_engine::ui::ViewInternalSetup for #name <#type_params>  {
            fn __internal_setup(&mut self) {
                use test_engine::ui::ViewSetup;
                use test_engine::ui::WithHeader;
                use test_engine::ui::ViewData;
                self.view.label += &#name_str.to_string();
                self.layout_header();
                let weak = test_engine::refs::weak_from_ref(self);
                weak.__link();
                weak.setup();
                self.__after_setup_event().trigger(());
            }
        }

        impl #generics  #name <#type_params> {
            fn __link(mut self: test_engine::refs::Weak<Self>) {
                #links
            }
        }

        impl #generics std::ops::Deref for #name <#type_params> {
            type Target = test_engine::ui::ViewBase;
            fn deref(&self) -> &test_engine::ui::ViewBase {
                &self.view
            }
        }
        impl #generics std::ops::DerefMut for #name <#type_params>  {
            fn deref_mut(&mut self) -> &mut test_engine::ui::ViewBase {
                &mut self.view
            }
        }
    }
    .into()
}

fn add_inits(root_name: &Ident, fields: &mut FieldsNamed) -> TokenStream2 {
    let subview = Ident::new("Sub", Span::call_site());

    let mut res = quote!();

    for field in &mut fields.named {
        let name = field.ident.as_ref().expect("let name = field.ident.as_ref()");

        if let Type::Path(path) = &field.ty {
            for segment in &path.path.segments {
                if segment.ident == subview {
                    let label = TokenStream2::from_str(&format!("\"{root_name}.{name}\""))
                        .expect("let label = TokenStream2::from_str()");

                    res = quote! {
                        #res
                        self.#name = self.add_view();
                        self.#name.label = format!("{}: {}", #label, self.#name.label);
                    }
                }
            }
        }
    }

    res
}

fn add_links(fields: &mut FieldsNamed) -> TokenStream2 {
    let mut res = quote!();

    for field in &mut fields.named {
        let field_name = field.ident.as_ref().expect("let field_name = field.ident.as_ref()");

        let attrs = field.attrs.clone();

        let attr = attrs.first();
        let Some(attr) = attr else {
            continue;
        };

        let attribute_name = attr.path.to_token_stream().to_string();
        let tokens = attr.tokens.to_token_stream().to_string();

        // Skip other macro. Should be smarter than that.
        let Some(method) = tokens.strip_prefix("= ") else {
            continue;
        };

        field.attrs = vec![];

        let parameter = Ident::new(method, Span::call_site());

        match attribute_name.as_str() {
            "link" => {
                res = quote! {
                    #res
                    {
                        use test_engine::ui::AlertErr;
                        self.#field_name.on_tap(move || { self.#parameter().alert_err(); });
                    }
                };
            }
            "link_async" => {
                res = quote! {
                    #res
                    self.#field_name.on_tap(move || {
                        tokio::spawn(async move {
                            use test_engine::ui::AlertErr;
                            self.#parameter().await.alert_err();
                        });
                    });
                };
            }
            "text" => {
                let param_str = TokenStream2::from_str(&format!("\"{parameter}\""))
                    .expect("let param_str = TokenStream2::from_str(");

                res = quote! {
                    #res
                    self.#field_name.set_text(#param_str);
                };
            }
            _ => panic!("Invalid attribute. Only `link`, 'link_async' and 'text' are supported."),
        }
    }

    res
}

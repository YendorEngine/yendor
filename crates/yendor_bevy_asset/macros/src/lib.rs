use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_quote, spanned::Spanned};

/// Derive macro for the `YendorBevyAsset` trait.
#[proc_macro_derive(YendorBevyAsset, attributes(asset_id, asset))]
pub fn yendor_bevy_asset(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap();

    impl_yendor_bevy_asset(&input).into()
}

fn impl_yendor_bevy_asset(input: &syn::DeriveInput) -> TokenStream2 {
    let deserialize_only: syn::Attribute = parse_quote! {
        #[asset(deserialize_only)]
    };
    let item_ident = &input.ident;

    let mut asset_id = None;
    for attr in &input.attrs {
        let Ok(syn::Meta::NameValue(name_value)) = attr.parse_meta() else {
            continue;
        };

        if name_value.path.get_ident().map(|i| i != "asset_id").unwrap_or(true) {
            continue;
        }

        let syn::Lit::Str(lit_str) = name_value.lit else {
            continue;
        };

        asset_id = Some(lit_str.value());
    }

    let Some(asset_id) = asset_id else {
        return quote! {
            compile_error!("You must specify a `asset_id` attribute");
        };
    };

    let module_ident = format_ident!(
        "{}_derive_bevy_asset",
        item_ident.to_string().to_lowercase()
    );

    // Parse the struct
    let in_struct = match &input.data {
        syn::Data::Struct(s) => s,
        syn::Data::Enum(_) | syn::Data::Union(_) => {
            return quote_spanned! { input.ident.span() =>
                compile_error!("You may only derive HasLoadProgress on structs");
            };
        },
    };

    let mut field_loads = Vec::new();
    'field: for field in &in_struct.fields {
        // Skip this field if it has `#[has_load_progress(none)]`
        for attr in &field.attrs {
            if attr.path == parse_quote!(asset) {
                if attr != &deserialize_only {
                    field_loads.push(quote_spanned! { attr.span() =>
                        compile_error!("Attribute must be `#[asset(deserialize_only)]` if specified");
                    });
                }
                continue 'field;
            }
        }
        let field_ident = field.ident.as_ref().expect("Field identifier missing");
        field_loads.push(quote_spanned! { field_ident.span() =>
            ::yendor_bevy_asset::YendorBevyAssetLoad::load(
                &mut meta.#field_ident,
                load_context,
                &mut dependencies
            );
        });
    }

    quote! {
        mod #module_ident {
            use ::yendor_type_ulid::TypeUlid;
            use ::bevy::asset::AddAsset;
            use super::#item_ident;

            // Make sure `TypeUlid` is implemented
            trait RequiredBounds: yendor_type_ulid::TypeUlid + for<'de> ::serde::Deserialize<'de> {}
            impl RequiredBounds for #item_ident {}

            impl ::bevy::reflect::TypeUuid for #item_ident {
                const TYPE_UUID: bevy::reflect::Uuid = bevy::reflect::Uuid::from_u128(Self::ULID.0);
            }

            struct AssetLoader;
            impl ::bevy::asset::AssetLoader for AssetLoader {
                fn load<'a>(
                    &'a self,
                    bytes: &'a [u8],
                    load_context: &'a mut bevy::asset::LoadContext,
                ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
                    Box::pin(async move {
                        let mut dependencies = Vec::new();
                        let mut meta: #item_ident =
                            if load_context.path().extension() == Some(std::ffi::OsStr::new("json")) {
                                ::yendor_bevy_asset::_private::serde_json::from_slice(bytes)?
                            } else {
                                ::yendor_bevy_asset::_private::serde_yaml::from_slice(bytes)?
                            };

                        #(#field_loads)*

                        load_context.set_default_asset(
                            bevy::asset::LoadedAsset::new(meta)
                                .with_dependencies(dependencies)
                        );

                        Ok(())
                    })
                }

                fn extensions(&self) -> &[&str] {
                    &[
                        concat!(#asset_id, ".json"),
                        concat!(#asset_id, ".yaml"),
                        concat!(#asset_id, ".yml"),
                    ]
                }
            }

            impl ::yendor_bevy_asset::YendorBevyAsset for #item_ident {
                fn install_asset(app: &mut ::bevy::app::App) {
                    app
                        .add_asset::<Self>()
                        .add_asset_loader(AssetLoader);
                }
            }
        }
    }
}

/// Derive macro for the `YendorBevyAssetLoad` trait.
#[proc_macro_derive(YendorBevyAssetLoad, attributes(asset))]
pub fn yendor_bevy_asset_load(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap();

    impl_yendor_bevy_asset_load(&input).into()
}

fn impl_yendor_bevy_asset_load(input: &syn::DeriveInput) -> TokenStream2 {
    let deserialize_only: syn::Attribute = parse_quote! {
        #[asset(deserialize_only)]
    };
    let item_ident = &input.ident;

    // Parse the struct
    let mut field_loads = Vec::new();
    match &input.data {
        syn::Data::Struct(s) => {
            'field: for field in &s.fields {
                // Skip this field if it has `#[has_load_progress(none)]`
                for attr in &field.attrs {
                    if attr.path == parse_quote!(asset) {
                        if attr != &deserialize_only {
                            field_loads.push(quote_spanned! { attr.span() =>
                                compile_error!("Attribute must be `#[asset(deserialize_only)]` if specified");
                            });
                        }
                        continue 'field;
                    }
                }
                let field_ident = field.ident.as_ref().expect("Field identifier missing");
                field_loads.push(quote_spanned! { field_ident.span() =>
                    ::yendor_bevy_asset::YendorBevyAssetLoad::load(
                        &mut self.#field_ident,
                        load_context,
                        dependencies
                    );
                });
            }
        },
        syn::Data::Enum(e) => {
            let mut patterns = Vec::new();
            for variant in &e.variants {
                let variant_ident = &variant.ident;
                match &variant.fields {
                    syn::Fields::Named(fields) => {
                        let ids = fields
                            .named
                            .iter()
                            .map(|x| x.ident.as_ref().expect("Field without ident"))
                            .collect::<Vec<_>>();
                        let loads = ids.iter().map(|id| {
                            quote! {
                                ::yendor_bevy_asset::YendorBevyAssetLoad::load(#id, load_context, dependencies);
                            }
                        });

                        patterns.push(quote! {
                            Self::#variant_ident { #(#ids,)* } => {
                                #(#loads)*
                            }
                        });
                    },
                    syn::Fields::Unnamed(fields) => {
                        let ids = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(i, _)| format_ident!("field_{}", i))
                            .collect::<Vec<_>>();
                        let loads = ids.iter().map(|id| {
                            quote! {
                                ::yendor_bevy_asset::YendorBevyAssetLoad::load(#id, load_context, dependencies);
                            }
                        });

                        patterns.push(quote! {
                            Self::#variant_ident(#(#ids)*) => {
                                #(#loads)*
                            }
                        });
                    },
                    syn::Fields::Unit => patterns.push(quote! {
                        Self::#variant_ident => (),
                    }),
                }
            }

            field_loads.push(quote! {
                match self {
                    #(#patterns)*
                }
            });
        },
        syn::Data::Union(_) => {
            return quote_spanned! { input.ident.span() =>
                compile_error!("Deriving not supported on unions");
            };
        },
    };

    quote! {
        impl ::yendor_bevy_asset::YendorBevyAssetLoad for #item_ident {
            fn load(
                &mut self,
                load_context: &mut bevy::asset::LoadContext,
                dependencies: &mut Vec<bevy::asset::AssetPath<'static>>,
            ) {
                #(#field_loads)*
            }
        }
    }
}

/// Derive macro for `HasLoadProgress`
///
/// May be used to implement `HasLoadProgress` on structs where all fields implement
/// `HasLoadProgress`.
///
/// Fields not implementing `HasLoadProgress` may be skipped with `#[has_load_progress(none)]`
/// added to the field.
///
/// `#[has_load_progress(none)]` may also be added to the struct itself to use the default
/// implementation of `HasLoadProgress` which returns no progress and nothing to load.
///
/// `#[has_load_progress(none)]` could also be added to enums to derive the default
/// implementation with no load progress, but cannot be added to enum variants.
#[proc_macro_derive(HasLoadProgress, attributes(has_load_progress))]
pub fn has_load_progress(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap();

    impl_has_load_progress(&input).into()
}

fn impl_has_load_progress(input: &syn::DeriveInput) -> TokenStream2 {
    // The attribute that may be added to skip fields or use the default implementation.
    let no_load_attr: syn::Attribute = parse_quote! {
        #[has_load_progress(none)]
    };

    let item_ident = &input.ident;
    let mut impl_function_body = quote! {};

    // Check for `#[has_load_progress(none)]` on the item itself
    let mut skip_all_fields = false;
    for attr in &input.attrs {
        if attr.path == parse_quote!(has_load_progress) {
            if attr == &no_load_attr {
                skip_all_fields = true;
            } else {
                return quote_spanned!(attr.span() =>
                    compile_error!("Attribute must be `#[has_load_progress(none)]` if specified");
                );
            }
        }
    }

    // If we are skipping all fields
    if skip_all_fields {
        impl_function_body = quote! {
            yendor_bevy_asset::LoadProgress::default()
        };

    // If we should process struct fields
    } else {
        // Parse the struct
        let in_struct = match &input.data {
            syn::Data::Struct(s) => s,
            syn::Data::Enum(_) | syn::Data::Union(_) => {
                return quote_spanned! { input.ident.span() =>
                    compile_error!("You may only derive HasLoadProgress on structs");
                };
            },
        };

        // Start a list of the progresses for each field
        let mut progresses = Vec::new();
        'field: for field in &in_struct.fields {
            // Skip this field if it has `#[has_load_progress(none)]`
            for attr in &field.attrs {
                if attr.path == parse_quote!(has_load_progress) {
                    if attr == &no_load_attr {
                        continue 'field;
                    } else {
                        impl_function_body = quote_spanned! { attr.span() =>
                            compile_error!("Attribute be `#[has_load_progress(none)]` if specified");
                        }
                    }
                }
            }

            // Add this fields load progress to the list of progresses
            let field_ident = field.ident.as_ref().expect("Field ident");
            progresses.push(quote_spanned! { field_ident.span() =>
                yendor_bevy_asset::HasLoadProgress::load_progress(
                    &self.#field_ident,
                    loading_resources
                )
            })
        }

        // Retrun the merged progress result
        impl_function_body = quote! {
            #impl_function_body
            yendor_bevy_asset::LoadProgress::merged([ #( #progresses),* ])
        };
    }

    // Fill out rest of impl block
    quote! {
        impl yendor_bevy_asset::HasLoadProgress for #item_ident {
            fn load_progress(
                &self,
                loading_resources: &yendor_bevy_asset::LoadingResources
            ) -> yendor_bevy_asset::LoadProgress {
                #impl_function_body
            }
        }
    }
}

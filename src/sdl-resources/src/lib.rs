extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn bundle_resources(_: TokenStream) -> TokenStream {
	//should produce a Sprites enum and Resources struct using the 'resources' folder
	TokenStream::new()
}
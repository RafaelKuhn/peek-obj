mod utils;

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn a_proc_macro(_input: TokenStream) -> TokenStream {
	"5".parse().unwrap()
}

#[proc_macro]
pub fn log_tokenc(input: TokenStream) -> TokenStream {
	
	let x = format!(r#"
		fn log() {{
			println!("entering");
			println!("args: {{}}", {args});
		}}
	"#,
	args = input.into_iter().count());
	x.parse().unwrap()
}

#[proc_macro]
pub fn log_tokens(input: TokenStream) -> TokenStream {

	let input_str = format!("\"{}\"", input);
	let x = format!(r#"
		println!("args: {{}}", {args});
	"#,
	args = input_str);
	x.parse().unwrap()
}

#[proc_macro]
pub fn xy_lin(input: TokenStream) -> TokenStream {

	let _count = input.clone().into_iter().count();

	let mut iter = input.into_iter();
	let _x = iter.next().expect("expecting a X value");
	iter.next().expect("expecting a separator");

	// let input_str = format!("\"{}\"", input.to_string());
	let x = r#"
		println!("args: {}", "smth");
	"#.to_string();
	x.parse().unwrap()
}


// pub(crate) use xy_to_lin;
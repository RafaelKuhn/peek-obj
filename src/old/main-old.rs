
use std::env;


fn main() {
	
    print!("Hello, world!\n\n");
	// let argsIterator: Args = args();
	// let args_iterator: env::Args = env::args();

	// print!("args: {:?}\n", args_iterator);
	
	// let args_vector: Vec<_> = env::args().skip(1).collect();
	// print!("args: {:?}\n", args_vector);

	// for (i, arg) in args_iterator.skip(1).enumerate() {
	// 	print!("arg {}: {}\n", i, arg);
	// }

	// args_iterator.skip(1).enumerate().for_each(|(i, arg)| {
	// 	print!("f arg {}: {} \n", i, arg);
	// })

	// let next = args.next();
	// if Ok(next) {
	// 	println!("1 arg: {}", next);
	// }



	let options = parse_options(&mut env::args());

	// let mut has_c_option = false;
	// let mut has_debug_option = false;
	// for (_i, arg) in args_iterator.skip(1).enumerate() {
	// 	match arg.as_str() {
	// 		"-d" | "--debug"  => has_debug_option = true,
	// 		"-c" | "--create" => has_c_option = true,
	// 		_ => continue,
	// 	}
	// 	// if arg == "-c" {
	// 	// 	has_c_option = true;
	// 	// 	continue;
	// 	// }
	// }

	// println!("has -d? {} ", has_debug_option);
	println!("has -d? {:#?} ", options);
}

// fn parse_options(args: env::Args) {
fn parse_options(args: &mut impl Iterator<Item = String>) {
	for arg in args {
		println!("{arg}");
	}

}

// fn parse_options(args: &Vec<String>) {
// 	// let mut options = Default::default();
// 	let options = Default::default();
// 	return options;
// }

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Default)]
pub struct Options {
	has_debug_option: bool,
	has_c_option: bool,

}

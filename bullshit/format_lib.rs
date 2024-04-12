

fn main() {

	// pad integer left
	let i = 74;
	println("{: <4}", i);
	//         ‖‖‖ 
	//         ‖‖+- width
	//         ‖+-- align
	//         +--- fill

	// pad float, force + sign, two decimals
	let f = 125.5643;
	println("{:+.2}", f);

}
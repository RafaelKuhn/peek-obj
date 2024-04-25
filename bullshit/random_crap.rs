

fn random_skip_ball() {
	let interval = 2.;
	let time = ((timer.time_aggr.as_millis() as f32 * 0.001 / interval) * 1.000) as u64;

	let seed = Seed::unsafe_new(time);
	let random = Random::from_seed(seed);

	let random_skip = (random.u32() as usize) % (yade_data.balls.len() - 1);
}


fn order_vec(yade_data: YadeDemData) {
	let mut copy = yade_data.balls.to_owned();

	buf.write_debug(&format!("  UNSORTED! \n"));
	copy.iter().enumerate().for_each(|(f,a)| buf.write_debug(&format!(" -> {:}: {:}\n", f, a)));

	copy.sort_by(|circ, other| {
		circ.pos.x.partial_cmp(&other.pos.x).unwrap()
	});
	
	buf.write_debug(&format!("  SORTED! \n"));
	copy.iter().enumerate().for_each(|(f,a)| buf.write_debug(&format!(" -> {:}: {:}\n", f, a)));
}


fn test_shit2() {

	let mut buf = [
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
		
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
		
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,

		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
	];

	let mut stdout = stdout();
	
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[0..16]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[4..16]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[8..16]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[12..16]);
	
	// BACKGROUND_FILL_CHAR.encode_utf8(&mut slice[1..4]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[16..32]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[20..32]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[24..32]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[28..32]);
	// BACKGROUND_FILL_CHAR.encode_utf8(&mut slice[5..8]);

	
	// let buf_str = unsafe { std::str::from_utf8_unchecked(&slice) };
	let buf_str = std::str::from_utf8(&buf[0..16]).unwrap();
	queue!(stdout, MoveTo(0, 0)).unwrap();
	queue!(stdout, Print(buf_str)).unwrap();
	
	let buf_str = std::str::from_utf8(&buf[16..32]).unwrap();
	queue!(stdout, MoveTo(0, 1)).unwrap();
	queue!(stdout, Print(buf_str)).unwrap();

	stdout.flush().unwrap();

	let _buf = vec![
		b'a', b'a', b'a', b'a', //  0  1  2  3
		b'b', 0, 0, 0, //  4  5  6  7
		b'c', 0, 0, 0, //  8  9 10 11
		b' ', 0, 0, 0, // 12 13 14 15
		0,    0, 0, 0, // 16 17 18 19
		// invalid utf8 example
		// 0xf0, 0x28, 0x8c, 0xbc,
		// 128, 223,
	];

	// '💖'.encode_utf8(&mut buf[16..20]);

}

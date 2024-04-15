
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

use riscv_decode::decode;

mod util;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let radix = if args[1] == "-b" {10} else {16};
	match u32::from_str_radix(args[2].trim_start_matches("0x"), radix) {
		Ok(raw_instruction) => match decode(raw_instruction) {
			Ok(instruction) => util::print(instruction),
			Err(error) => print!("DecodingError: {:?}", error),
		},
		Err(error) => print!("ParseIntError: {:?}", error),
	}
}

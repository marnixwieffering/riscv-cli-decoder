use riscv_decode::decode;

mod util;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	match u32::from_str_radix(args[1].trim_start_matches("0x"), 16) {
		Ok(raw_instruction) => match decode(raw_instruction) {
			Ok(instruction) => util::print(instruction),
			Err(error) => print!("DecodingError: {:?}", error),
		},
		Err(error) => print!("ParseIntError: {:?}", error),
	}
}

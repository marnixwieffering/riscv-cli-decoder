use std::env;
use std::result::Result;

use clap::{crate_version, App, Arg};
use riscv_decode::decode;

mod util;

fn main() -> Result<(), std::io::Error> {
	let matches = App::new("riscv-cli-decoder")
		.version(crate_version!())
		.version("1.0")
		.author("Marnix Wieffering. <git@marnixwieffering.dev>")
		.about("Decodes RISCV instructions")
		.arg(
			Arg::with_name("decode")
				.short("d")
				.value_name("instruction")
				.takes_value(true)
				.required(true),
		)
		.get_matches();

	let input = matches.value_of("decode").unwrap();
	let result = decode(u32::from_str_radix(input.trim_start_matches("0x"), 16).unwrap()).unwrap();
	util::print(result);

	return Ok(());
}

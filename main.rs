// #[allow(unused_variables)]
// use std::env;
use std::process;
use std::fs;

fn error_handling(args: &Vec<String>) {
	let num_args = args.len();
	if args.is_empty() || num_args > 3 {
		eprintln!("usage: ft_ssl command [flags] [file/string]");
		process::exit(1);
	}
	else if num_args < 3 {
		eprintln!("ft_ssl: Error: 'foobar' is an invalid command.\n");
		eprintln!("Commands:");
		eprintln!("md5");
		eprintln!("sha256\n");
		eprintln!("Flags:");
		eprintln!("-p -q -r -s");
		process::exit(1);
	}
}

fn count_msg_size(msg: &String) -> u64 {
	let msg_metadata = fs::metadata(msg).expect("Failed to get msg metadata");
	msg_metadata.len()
}

fn main ()
{
	let args: Vec<String> = std::env::args().skip(1).collect();
	error_handling(&args);
	let msg_size = count_msg_size(&args[2]);
	println!("{}", msg_size);
	//dbg!(args);
}
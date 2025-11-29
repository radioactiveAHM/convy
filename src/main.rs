mod arg;
mod process;
mod threading;

fn main() {
	let args = arg::Args::new();

	if args.input.is_file() {
		// single file proccessing
		process::process(&args, &args.input).unwrap();
	} else if args.input.is_dir() {
		// multi file proccessing
		threading::with_threads(args);
	} else {
		panic!("invalid input");
	}
}

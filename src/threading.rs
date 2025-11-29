pub fn with_threads(args: crate::arg::Args) {
	if args.output.is_file() {
		panic!("bulk processing requires the output path to be a directory.")
	}

	let dir_entries: Vec<std::fs::DirEntry> = std::fs::read_dir(&args.input).unwrap().filter_map(Result::ok).collect();
	let threads = threads(args.threads);

	let mut handles = Vec::with_capacity(threads);
	let arc_args = std::sync::Arc::new(args);
	for chunck in dir_entries.chunks(dir_entries.len().div_ceil(threads)) {
		let args = arc_args.clone();
		let dir_entries = chunck.iter().map(|entry| entry.path()).collect();
		handles.push(std::thread::spawn(|| {
			let args = args;
			crate::process::bulk_process(dir_entries, &args);
		}));
	}
	for handle in handles {
		handle.join().unwrap();
	}
}

fn threads(n: usize) -> usize {
	if n == 0 {
		std::thread::available_parallelism()
			.unwrap_or(std::num::NonZero::new(1).unwrap())
			.get()
	} else {
		n
	}
}

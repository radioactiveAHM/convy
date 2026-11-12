pub fn with_threads(args: crate::arg::Args) {
	if args.output.is_file() {
		panic!("bulk processing requires the output path to be a directory.")
	}

	let dir_entries = std::fs::read_dir(&args.input)
		.unwrap()
		.filter_map(Result::ok)
		.collect::<Vec<std::fs::DirEntry>>();

	let dir_entries = std::sync::Arc::new(std::sync::Mutex::new(dir_entries));
	let threads = threads(args.threads);

	let mut handles = Vec::with_capacity(threads);
	let arc_args = std::sync::Arc::new(args);
	for _ in 0..threads {
		let args = arc_args.clone();
		let dir_entries = dir_entries.clone();
		handles.push(std::thread::spawn(move || {
			loop {
				// if let pattern holds the lock
				let entry_option = { dir_entries.lock().unwrap().pop() };

				if let Some(entry) = entry_option {
					let entry_path = entry.path();
					if entry_path.is_file()
						&& let Err(e) = crate::process::process(&args, &entry_path)
					{
						println!("{e}");
					}
				} else {
					break;
				}
			}
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

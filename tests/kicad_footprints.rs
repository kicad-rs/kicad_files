use kicad_files::board::Footprint;
use std::{
	fs::{self, File},
	io::Read as _,
	panic::catch_unwind,
	path::{Path, PathBuf},
	thread::available_parallelism
};

use anyhow::bail;
use libtest::{Arguments, Failed, Trial};

struct TestData {
	path: PathBuf
}

fn run_test(data: &TestData) -> Result<(), Failed> {
	let mut file = File::open(&data.path)?;
	let mut input = String::new();
	file.read_to_string(&mut input)
		.map_err(|err| format!("Failed to read input file: {err}"))?;
	drop(file);

	input
		.parse::<Footprint>()
		.map(|_| ())
		.map_err(|err| format!("{err:?}").into())
}

fn add_tests_from_dir<C, P>(
	cargo_dir: C,
	tests: &mut Vec<Trial>,
	path: P
) -> anyhow::Result<()>
where
	C: AsRef<Path> + Copy,
	P: AsRef<Path>
{
	for entry in fs::read_dir(path)? {
		let entry = entry?;
		let path = entry.path();
		let Some(ext) = path.extension() else {
			continue;
		};
		let file_type = entry.file_type()?;

		if file_type.is_dir() && ext == "pretty" {
			add_tests_from_dir(cargo_dir, tests, path)?;
		} else if file_type.is_file() && ext == "kicad_mod" {
			let name = path.strip_prefix(cargo_dir)?.display().to_string();
			tests.push(Trial::test(name, move || {
				let data = TestData { path };

				match catch_unwind(|| run_test(&data)) {
					Ok(result) => result,
					Err(_) => Err(Failed::without_message())
				}
			}))
		}
	}

	Ok(())
}

fn main() -> anyhow::Result<()> {
	let mut args = Arguments::from_args();

	// we are heavily bottleneck'ed on i/o
	// to improve performance, we increase the number of threads, unless specified,
	// above the number of cores
	// ideally we'd use async, but I don't think libtest-mimic supports that
	if args.test_threads.is_none() {
		args.test_threads = Some(available_parallelism()?.get() * 4)
	}

	let cargo_dir: PathBuf = env!("CARGO_MANIFEST_DIR").parse().unwrap();
	let dir = cargo_dir.join("tests").join("kicad-footprints");

	let mut tests = Vec::new();
	add_tests_from_dir(&cargo_dir, &mut tests, dir)?;
	if tests.is_empty() {
		bail!("No test data found, did you forget to initialize git submodules?");
	}

	libtest::run(&args, tests).exit()
}

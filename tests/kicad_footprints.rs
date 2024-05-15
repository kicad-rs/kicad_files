use anyhow::bail;
use futures_util::FutureExt as _;
use kicad_files::board::Footprint;
use libtest::{Arguments, Failed, Trial};
use std::{
	fs,
	path::{Path, PathBuf},
	sync::Arc
};
use tokio::runtime::Runtime;

async fn run_test<P: AsRef<Path>>(path: P) -> Result<(), Failed> {
	let input = tokio::fs::read_to_string(path)
		.await
		.map_err(|err| format!("Failed to read input file: {err}"))?;
	input
		.parse::<Footprint>()
		.map(|_| ())
		.map_err(|err| format!("{err:?}").into())
}

fn add_tests_from_dir<C, P>(
	rt: Arc<Runtime>,
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
			add_tests_from_dir(Arc::clone(&rt), cargo_dir, tests, path)?;
		} else if file_type.is_file() && ext == "kicad_mod" {
			let name = path.strip_prefix(cargo_dir)?.display().to_string();
			let rt = Arc::clone(&rt);
			tests.push(Trial::test(name, move || {
				rt.block_on(async move {
					match run_test(&path).catch_unwind().await {
						Ok(result) => result,
						Err(_) => Err(Failed::without_message())
					}
				})
			}))
		}
	}

	Ok(())
}

fn main() -> anyhow::Result<()> {
	let mut args = Arguments::from_args();

	let rt = Arc::new(Runtime::new()?);

	let cargo_dir: PathBuf = env!("CARGO_MANIFEST_DIR").parse().unwrap();
	let dir = cargo_dir.join("tests").join("kicad-footprints");

	let mut tests = Vec::new();
	add_tests_from_dir(rt, &cargo_dir, &mut tests, dir)?;
	if tests.is_empty() {
		bail!("No test data found, did you forget to initialize git submodules?");
	}

	// tokio will limit the number of threats, so we want all of them
	if args.test_threads.is_none() {
		args.test_threads = Some(tests.len());
	}

	libtest::run(&args, tests).exit()
}

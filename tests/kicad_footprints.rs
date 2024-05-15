use anyhow::{anyhow, bail};
use futures_util::TryStreamExt as _;
use kicad_files::board::Footprint;
use libtest::{Arguments, Failed, Trial};
use std::{
	panic::catch_unwind,
	path::{Path, PathBuf},
	sync::Arc
};
use tokio::{runtime::Runtime, sync::Mutex};
use tokio_stream::wrappers::ReadDirStream;

fn run_test(input: String) -> Result<(), Failed> {
	input
		.parse::<Footprint>()
		.map(|_| ())
		.map_err(|err| format!("{err:?}").into())
}

async fn add_tests_from_dir<C, P>(
	cargo_dir: C,
	tests: &mut Vec<Trial>,
	path: P
) -> anyhow::Result<()>
where
	C: AsRef<Path> + Copy,
	P: AsRef<Path>
{
	let tests = Arc::new(Mutex::new(tests));
	let entries = ReadDirStream::new(tokio::fs::read_dir(path).await?);
	entries
		.map_err(anyhow::Error::from)
		.try_for_each_concurrent(None, |entry| {
			let tests = Arc::clone(&tests);
			async move {
				let path = entry.path();
				let Some(ext) = path.extension() else {
					return Ok(());
				};
				let file_type = entry.file_type().await?;

				if file_type.is_dir() && ext == "pretty" {
					let mut tests = tests.lock().await;
					add_tests_from_dir(cargo_dir, &mut tests, path).await?;
				} else if file_type.is_file() && ext == "kicad_mod" {
					let name = path.strip_prefix(cargo_dir)?.display().to_string();
					let input =
						tokio::fs::read_to_string(path).await.map_err(|err| {
							anyhow!("Failed to read input file: {err}")
						})?;
					tests.lock().await.push(Trial::test(name, move || {
						match catch_unwind(|| run_test(input)) {
							Ok(result) => result,
							Err(_) => Err(Failed::without_message())
						}
					}))
				}

				anyhow::Ok(())
			}
		})
		.await?;

	Ok(())
}

fn main() -> anyhow::Result<()> {
	let args = Arguments::from_args();

	let rt = Runtime::new()?;

	let cargo_dir: PathBuf = env!("CARGO_MANIFEST_DIR").parse().unwrap();
	let dir = cargo_dir.join("tests").join("kicad-footprints");

	let mut tests = Vec::new();
	rt.block_on(add_tests_from_dir(&cargo_dir, &mut tests, dir))?;
	if tests.is_empty() {
		bail!("No test data found, did you forget to initialize git submodules?");
	}

	// tokio will limit the number of threats, so we want all of them
	// if args.test_threads.is_none() {
	// 	args.test_threads = Some(tests.len());
	// }

	libtest::run(&args, tests).exit()
}

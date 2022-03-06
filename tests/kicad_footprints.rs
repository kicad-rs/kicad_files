use kicad_files::board::Footprint;
use std::{
	fs::{self, File},
	io::{self, Read as _, Write as _},
	path::PathBuf,
	time::Instant
};

mod util {
	include!("util/mod.rs");
}
use util::*;

#[test]
fn test_deserialize_kicad_footprints() -> io::Result<()> {
	let mut stdout = init_stdout();

	let cargo_dir: PathBuf = env!("CARGO_MANIFEST_DIR").parse().unwrap();
	let dir = cargo_dir.join("tests").join("kicad-footprints");

	let mut fp_count = 0;
	let mut fp_fail = 0;
	for entry in fs::read_dir(&dir).unwrap().map(Result::unwrap) {
		let path = entry.path();
		match path.extension() {
			Some(ext) if ext == "pretty" => {
				write!(
					stdout,
					"\ttest {} ...",
					path.strip_prefix(&dir).unwrap().display()
				)?;
				stdout.flush()?;

				let timer = Instant::now();
				let mut ok = true;
				for entry in fs::read_dir(path).unwrap().map(Result::unwrap) {
					let path = entry.path();
					match path.extension() {
						Some(ext) if ext == "kicad_mod" => {
							let mut file = File::open(&path)?;
							let mut input = String::new();
							file.read_to_string(&mut input)?;
							drop(file);

							fp_count += 1;
							if let Err(err) = Footprint::from_str(&input) {
								if ok {
									write_fail(&mut stdout)?;
								}
								ok = false;
								fp_fail += 1;
								red(&mut stdout)?;
								writeln!(
									stdout,
									"\t\t{}: {:?}",
									path.strip_prefix(&cargo_dir).unwrap().display(),
									err
								)?;
								white(&mut stdout)?;
							}
						},
						_ => {}
					};
				}

				if ok {
					write_ok(&mut stdout, timer.elapsed())?;
				}
			},
			_ => {}
		};
	}

	if fp_count == 0 {
		panic!("No test data found, did you forget to initialize git submodules?");
	}

	write_summary(&mut stdout, fp_count, fp_fail)?;

	if fp_fail != 0 {
		panic!("fail");
	}

	Ok(())
}

use kicad_files::board::Footprint;
use std::{
	fs::{self, File},
	io::{self, Read as _, Write as _},
	path::PathBuf
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor as _};

fn write_ok(stdout: &mut StandardStream) -> io::Result<()> {
	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
	writeln!(stdout, " ok")?;
	stdout.set_color(ColorSpec::new().set_fg(None))?;
	Ok(())
}

fn write_fail(stdout: &mut StandardStream) -> io::Result<()> {
	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
	writeln!(stdout, " fail")?;
	stdout.set_color(ColorSpec::new().set_fg(None))?;
	Ok(())
}

#[test]
fn test_deserialize_kicad_footprints() -> io::Result<()> {
	let mut stdout = StandardStream::stdout(ColorChoice::Always);

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

				let mut ok = true;
				for entry in fs::read_dir(path).unwrap().map(Result::unwrap) {
					let path = entry.path();
					match path.extension() {
						Some(ext) if ext == "kicad_mod" => {
							let mut file = File::open(&path)?;
							let mut input = String::new();
							file.read_to_string(&mut input)?;
							drop(file);

							if input.starts_with("(footprint") {
								fp_count += 1;
								if let Err(err) = serde_sexpr::from_str(&input)
									.map(|_fp: Footprint| ())
								{
									if ok {
										write_fail(&mut stdout)?;
									}
									ok = false;
									fp_fail += 1;
									stdout.set_color(
										ColorSpec::new().set_fg(Some(Color::Red))
									)?;
									writeln!(
										stdout,
										"\t\t{}: {:?}",
										path.strip_prefix(&cargo_dir)
											.unwrap()
											.display(),
										err
									)?;
									stdout
										.set_color(ColorSpec::new().set_fg(None))?;
								}
							}
						},
						_ => {}
					};
				}

				if ok {
					write_ok(&mut stdout)?;
				}
			},
			_ => {}
		};
	}

	if fp_count == 0 {
		panic!("No test data found, did you forget to initialize git submodules?");
	}

	writeln!(stdout)?;
	write!(stdout, "\ttest result: ")?;
	if fp_fail == 0 {
		stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
		write!(stdout, "ok")?;
	} else {
		stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
		write!(stdout, "FAILED")?;
	}
	stdout.set_color(ColorSpec::new().set_fg(None))?;
	writeln!(
		stdout,
		". {} passed; {} failed",
		fp_count - fp_fail,
		fp_fail
	)?;
	writeln!(stdout)?;

	if fp_fail != 0 {
		panic!("fail");
	}

	Ok(())
}

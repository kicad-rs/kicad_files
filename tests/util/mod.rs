use std::{
	io::{self, Write as _},
	time::Duration
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor as _};

pub fn init_stdout() -> StandardStream {
	StandardStream::stdout(ColorChoice::Always)
}

pub fn green(stdout: &mut StandardStream) -> io::Result<()> {
	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))
}

pub fn red(stdout: &mut StandardStream) -> io::Result<()> {
	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))
}

pub fn white(stdout: &mut StandardStream) -> io::Result<()> {
	stdout.set_color(ColorSpec::new().set_fg(None))
}

pub fn write_ok(stdout: &mut StandardStream, duration: Duration) -> io::Result<()> {
	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
	write!(stdout, " ok")?;
	stdout.set_color(ColorSpec::new().set_fg(None))?;
	writeln!(stdout, " ({:.3} secs)", duration.as_secs_f32())?;
	Ok(())
}

pub fn write_fail(stdout: &mut StandardStream) -> io::Result<()> {
	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
	writeln!(stdout, " fail")?;
	stdout.set_color(ColorSpec::new().set_fg(None))?;
	Ok(())
}

pub fn write_summary(stdout: &mut StandardStream, fp_count: usize, fp_fail: usize) -> io::Result<()> {
	writeln!(stdout)?;
	write!(stdout, "\ttest result: ")?;
	if fp_fail == 0 {
		green(stdout)?;
		write!(stdout, "ok")?;
	} else {
		red(stdout)?;
		write!(stdout, "FAILED")?;
	}
	white(stdout)?;
	writeln!(
		stdout,
		". {} passed; {} failed",
		fp_count - fp_fail,
		fp_fail
	)?;
	writeln!(stdout)?;
	
	Ok(())
}

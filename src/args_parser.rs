use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ez-renamer")]
pub struct Args {
	/// take text from stdin
	#[structopt(name = "-")]
	pub take_from_stdin: bool,
}

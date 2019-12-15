use structopt::StructOpt;

#[structopt(name = "typer-rs")]
#[derive(StructOpt, Debug)]
pub struct Args {
    /// take text from stdin
    #[structopt(name = "-")]
    pub take_from_stdin: bool,
}

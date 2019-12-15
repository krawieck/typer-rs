use structopt::StructOpt;

#[structopt(name = "typer-rs")]
#[derive(StructOpt, Debug)]
pub struct Args {
    /// take text from stdin
    #[structopt(name = "-")]
    pub take_from_stdin: bool,

    /// text that should be then written by the user, or path to a file, or link
    /// to a website, the text should be downloaded from
    ///
    /// example:
    /// typer-rs -i "$(fortune)"
    #[structopt(short, long, default_value = "")]
    pub input: String,
}

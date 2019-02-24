use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(StructOpt, Debug)]
pub enum Subcommand {
    #[structopt(name = "binary")]
    Binary { count: u32 },

    #[structopt(name = "chars")]
    Chars { width: u32, height: u32 },

    #[structopt(name = "floats")]
    Floats {
        count: u32,
        precision: u32,
        min: f64,
        max: f64,
    },

    #[structopt(name = "ints")]
    Ints { count: u32, min: i64, max: i64 },

    #[structopt(name = "words")]
    Words { count: u32 },
}

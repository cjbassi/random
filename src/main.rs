mod args;

use std::fs;
use std::iter;

use rand::distributions::Alphanumeric;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;
use structopt::StructOpt;

use args::{Args, Subcommand};

const DICTIONARY: &str = "/usr/share/dict/cracklib-small";

fn main() {
    let args = Args::from_args();

    let mut rng = rand::thread_rng();

    match args.subcommand {
        Subcommand::Binary { count } => println!(
            "{}",
            iter::repeat(())
                .take(count as usize)
                .map(|()| [0, 1].choose(&mut rng).unwrap().to_string())
                .collect::<Vec<String>>()
                .concat()
        ),
        Subcommand::Chars { width, height } => {
            iter::repeat(()).take(height as usize).for_each(|()| {
                let chars: String = iter::repeat(())
                    .take(width as usize)
                    .map(|()| rng.sample(Alphanumeric))
                    .collect();
                println!("{}", chars);
            });
        }
        Subcommand::Floats {
            count,
            precision,
            min,
            max,
        } => {
            iter::repeat(()).take(count as usize).for_each(|()| {
                let float: f64 = rng.gen_range(min, max);
                println!("{1:.0$}", precision as usize, float);
            });
        }
        Subcommand::Ints { count, min, max } => {
            iter::repeat(())
                .take(count as usize)
                .for_each(|()| println!("{}", rng.gen_range(min, max)));
        }
        Subcommand::Words { count } => fs::read_to_string(DICTIONARY)
            .expect("failed to read dictionary")
            .lines()
            .choose_multiple(&mut rng, count as usize)
            .into_iter()
            .for_each(|word| println!("{}", word)),
    }
}

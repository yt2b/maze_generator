use crate::{digging_hole::DiggingHole, extending_wall::ExtendingWall, laying_stick::LayingStick};
use clap::Parser;
use generator::Generator;
use std::{thread, time::Duration};
mod digging_hole;
mod extending_wall;
mod generator;
mod laying_stick;
mod vector2;
mod walker;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Algorithm {
    Extending,
    Digging,
    Laying,
}

fn validate_len(s: &str) -> Result<usize, String> {
    let len: usize = s
        .parse()
        .map_err(|_| "5以上の奇数を指定してください".to_owned())?;
    if len >= 5 && len % 2 == 1 {
        Ok(len)
    } else {
        Err("5以上の奇数を指定してください".to_owned())
    }
}

#[derive(Debug, Parser)]
struct Args {
    #[clap(long, default_value = "extending")]
    algorithm: Algorithm,
    #[clap(long, default_value = "15")]
    #[arg(value_parser=validate_len)]
    height: usize,
    #[clap(long, default_value = "15")]
    #[arg(value_parser=validate_len)]
    width: usize,
}

fn main() {
    let args = Args::parse();
    let mut generator: Box<dyn Generator> = match args.algorithm {
        Algorithm::Extending => Box::new(ExtendingWall::new(args.height, args.width)),
        Algorithm::Digging => Box::new(DiggingHole::new(args.height, args.width)),
        Algorithm::Laying => Box::new(LayingStick::new(args.height, args.width)),
    };
    print!("\x1b[2J");
    loop {
        print!("\x1b[1;1H");
        for line in generator.get_field() {
            let s: String = line.iter().map(|&v| if v { '#' } else { '.' }).collect();
            println!("{}", s);
        }
        if !generator.in_process() {
            break;
        }
        generator.proceed();
        thread::sleep(Duration::from_millis(10));
    }
}

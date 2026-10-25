use std::io::{self, Write};
use std::process::ExitCode;
use std::str::FromStr;

mod game;
mod scoreboard;
mod config;
mod rng;
mod predictor;
mod opponent;

use crate::game::{Move, Round};
use crate::scoreboard::Scoreboard;
use crate::config::{Config, parse_config};
use crate::rng::XorShiftRng;
use crate::predictor::NGramPredictor;
use crate::opponent::Opponent;

enum Command {
    Play(Move),
    Quit,
}

#[derive(Debug)]
struct InvalidCommand;

impl FromStr for Command {
    type Err = InvalidCommand;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim() {
            "r" | "rock" => Ok(Command::Play(Move::Rock)),
            "p" | "paper" => Ok(Command::Play(Move::Paper)),
            "s" | "scissors" => Ok(Command::Play(Move::Scissors)),
            "q" | "quit" => Ok(Command::Quit),
            _ => Err(InvalidCommand),
        }
    }
}

fn read_command(stdin: &io::Stdin) -> io::Result<Option<Command>> {
    loop {
        print!("(r)ock, (p)aper, (s)cissors or (q)uit > ");
        io::stdout().flush()?;
        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            return Ok(None);
        }
        match line.parse::<Command>() {
            Ok(command) => return Ok(Some(command)),
            Err(InvalidCommand) => println!("unrecognized input"),
        }
    }
}

fn run(config: Config) -> io::Result<()> {
    let predictor = NGramPredictor::new(config.window_size(), config.use_backoff());
    let rng = XorShiftRng::from_system_time();
    let mut opponent = Opponent::new(predictor, rng);
    let mut scoreboard = Scoreboard::default();
    let stdin = io::stdin();

    while let Some(command) = read_command(&stdin)? {
        let Command::Play(player_move) = command else {
            break;
        };
        let opponent_move = opponent.play();
        let round = Round::new(player_move, opponent_move);
        scoreboard.record(round.outcome());
        opponent.observe(player_move);
        println!("{round}\n{scoreboard}\n");
    }
    Ok(())
}

fn main() -> ExitCode {
    let config = match parse_config(std::env::args().skip(1)) {
        Ok(config) => config,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(2);
        }
    };
    match run(config) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        },
    }
}

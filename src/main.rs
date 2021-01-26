use std::fmt;
use std::io::{self, Write};
use std::option::Option;

use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;

struct Name(Option<String>);

impl Name {
	fn new(input: Option<String>) -> Name {
		lazy_static! {
			static ref NAME_PATTERN: regex::Regex = Regex::new("^[A-Za-z]+$").unwrap(); // Only letters allowed for names
		}

		let sanitized = match input {
			Some(name) if NAME_PATTERN.is_match(&name) => Name(Some(name)),
			_ => Name(None), // Note: None is displayed as Anon
		};
		sanitized
	}
}

impl fmt::Display for Name {
	// If a name isn't picked, use "Anon".
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// A good result should be the string
		if let Option::Some(name) = &self.0 {
			write!(f, "{}", name)
		} else {
			// Otherwise return the string Invalid Name
			write!(f, "Anon")
		}
	}
}

#[derive(PartialEq)] // alows auto == comparisons
enum Choice {
	Rock,
	Paper,
	Scissors,
}

impl Choice {
	// Some(Choice) from input, None if invalid
	fn new(input: Option<String>) -> Option<Choice> {
		lazy_static! {
			static ref ROCK: regex::Regex = Regex::new("^[Rr]([Oo][Cc][Kk])?$").unwrap();
		}
		lazy_static! {
			static ref PAPER: regex::Regex = Regex::new("^[Pp]([Aa][Pp][Ee][Rr])?$").unwrap();
		}
		lazy_static! {
			static ref SCISSORS: regex::Regex =
				Regex::new("^[Ss]([Cc][Ii][Ss][Ss][Oo][Rr][Ss])?$").unwrap();
		}

		// return Option-ified choice based on some regex
		match input {
			Some(name) if ROCK.is_match(&name) => Some(Choice::Rock),
			Some(name) if PAPER.is_match(&name) => Some(Choice::Paper),
			Some(name) if SCISSORS.is_match(&name) => Some(Choice::Scissors),
			_ => None,
		}
	}

	// Computer choice
	fn random() -> Choice {
		let mut rng = rand::thread_rng();
		match rng.gen_range(0..3) {
			0 => Choice::Rock,
			1 => Choice::Paper,
			2 => Choice::Scissors,
			_ => panic!("Brian can't count..."),
		}
	}

	fn beat(&self, choice: &Choice) -> bool {
		match (self, choice) {
			(Choice::Rock, Choice::Scissors) => true,
			(Choice::Paper, Choice::Rock) => true,
			(Choice::Scissors, Choice::Paper) => true,
			_ => false,
		}
	}
}

impl fmt::Display for Choice {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Choice::Rock => write!(f, "Rock"),
			Choice::Paper => write!(f, "Paper"),
			Choice::Scissors => write!(f, "Scissors"),
		}
	}
}

fn read_input(prompt: &str) -> Option<String> {
	// Print the prompt then flush the print buffer, since auto flush is on '\n'
	print!("{}", prompt);
	// ignore errors
	if let Err(_) = io::stdout().flush() {
		return None;
	}

	// While it would be more efficent if we had a global buffer, this is simpler to use.
	// Plus we can trim() and other post processing on input with ease.
	let mut input_buffer: String = String::new();
	// Wait and block for next stdin from user, store within provided String buffer
	if let Err(_) = io::stdin().read_line(&mut input_buffer) {
		return None;
	};

	match input_buffer.trim().to_string() {
		empty if empty.is_empty() => None, // let callers of read_input handle None case
		name => Some(name),
	}
}

fn play(name: &Name, depth: usize) -> bool {
	// Force quit if 5 failed attempts at picking an option
	if depth >= 4 {
		println!("You are really bad at instructions");
		return false;
	}

	let choice: Choice = match Choice::new(read_input("\nR(ock), P(aper), or S(cissors) > ")) {
		None => {
			// return to not run the rest of play, play again to get an answer
			println!("Please type one of \"Rock\", \"Paper\", or \"Scissors\".");
			return play(name, depth + 1);
		}
		Some(choice) => choice, // extract choice from Option
	};
	let computer_choice = Choice::random(); // guarenteed to be a valid choice

	println!("\n{} chose: {}", name, choice);
	println!("Computer chose: {}\n", computer_choice);

	let win = choice.beat(&computer_choice);
	let lose = computer_choice.beat(&choice);

	match (win, lose) {
		(false, false) => println!("It's a Tie!"),
		(true, false) => println!("You win! {} beats {}", choice, computer_choice),
		(false, true) => println!("AI won!  Roko's coming for you with {}", computer_choice),
		(true, true) => panic!("Brian can't write logic"),
	}

	lazy_static! {
		static ref YES: regex::Regex = Regex::new("^[Yy]([Ee][Ss])?$").unwrap();
	}
	lazy_static! {
		static ref NO: regex::Regex = Regex::new("^[Nn]([Oo])?$").unwrap();
	}
	match read_input("Do you want to play again (y/[n]) > ") {
		Some(again) if YES.is_match(&again) => true,
		Some(again) if NO.is_match(&again) => false,
		_ => {
			println!("It is a smpiple yes or no question, and yet you still screwed that up.");
			false
		}
	}
}

fn main() {
	println!("Welcome to Rusty Rock Paper Scissors");

	let name = Name::new(read_input("What is your name > "));
	println!("Hello {}", name);

	// play until play returns false
	while play(&name, 0) {}
	println!("\nThanks for playing!");
}

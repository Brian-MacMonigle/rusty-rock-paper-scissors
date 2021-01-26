use std::io::{ self, Write };
use std::option::{ Option, };
use std::fmt;

use rand::Rng;
use lazy_static::lazy_static;
use regex::Regex;

struct Name(Option<String>);

impl Name {
	fn new(input: Option<String>) -> Name {
		lazy_static! { static ref NAME_PATTERN : regex::Regex = Regex::new("^[A-Za-z]+$").unwrap(); }

		let sanitized = match input {
			Some(name) if NAME_PATTERN.is_match(&name) => { Name(Some(name)) },
			_ => Name(None),
		};
		sanitized
	}
}

impl fmt::Display for Name {
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

#[derive(PartialEq)]
enum Choice {
	Rock,
	Paper,
	Scissors,
}

impl Choice {
	// Some(Choice) from input, None if invalid
	fn new(input: Option<String>) -> Option<Choice> {
		lazy_static! { static ref ROCK : regex::Regex = Regex::new("^[Rr][Oo][Cc][Kk]$").unwrap(); }
		lazy_static! { static ref PAPER : regex::Regex = Regex::new("^[Pp][Aa][Pp][Ee][Rr]$").unwrap(); }
		lazy_static! { static ref SCISSORS : regex::Regex = Regex::new("^[Ss][Cc][Ii][Ss][Ss][Oo][Rr][Ss]$").unwrap(); }

		let sanitized: Option<Choice> = match input {
			Some(name) if ROCK.is_match(&name) => Some(Choice::Rock),
			Some(name) if PAPER.is_match(&name) => Some(Choice::Paper),
			Some(name) if SCISSORS.is_match(&name) => Some(Choice::Scissors),
			_ => None,
		};
		sanitized
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
		if let Choice::Rock = self {
			write!(f, "Rock")
		} else if let Choice::Paper = self {
			write!(f, "Paper")
		} else if let Choice::Scissors = self {
			write!(f, "Scissors")
		} else {
			write!(f, "Unknown")
		}
	}
}

fn read_input (prompt: &str) -> Option<String> {
	// Print the prompt then flush the print buffer
	print!("{}", prompt);
	io::stdout().flush().expect("Unable to flush stdout");

	let mut input_buffer: String = String::new();
	// Wait and block for next stdin from user, store within provided String buffer
	io::stdin().read_line(&mut input_buffer).expect("Unable to read the line");

	return match input_buffer.trim().to_string() {
		empty if empty.is_empty() => Option::None,
		name => Option::Some(name),
	};
}

fn play (name: &Name) -> bool {
	let choice: Choice = match Choice::new(read_input("\nRock, Paper, or Scissors > ")) {
		None => {
			println!("Please type one of \"Rock\", \"Paper\", or \"Scissors\".");
			return play(name); // exit out of this function call
		}
		Some(choice) => choice
	};
	
	let computer_choice = Choice::random();

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

	lazy_static! { static ref YES : regex::Regex = Regex::new("^[Yy]([Ee][Ss])?$").unwrap(); }
	lazy_static! { static ref NO : regex::Regex = Regex::new("^[Nn]([Oo])?$").unwrap(); }
	let again = match read_input("Do you want to play again (y/[n]) > ") {
		Some(string) if YES.is_match(&string) => { true },
		Some(string) if NO.is_match(&string) => { false }, 
		_ => { 
			println!("It is a smpiple yes or no question, and yet you still screwed that up.");
			false
		},
	};
	return again;
}

fn main() {
	println!("Welcome to Rusty Rock Paper Scissors");

	let name = Name::new(read_input("What is your name > "));
	println!("Hello {}", name);

	loop {
		if !play(&name) {
			break;
		}
	}
	println!("\nThanks for playing!");
}
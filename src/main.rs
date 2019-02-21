#![feature(nll)]
#![feature(try_from)]

extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::convert::TryFrom;
use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeStruct};

mod language_model;
mod alphabet;

use language_model::*;
use alphabet::*;

fn main() {
	let mut l: LanguageModel = LanguageModel::new();

	for arg in std::env::args().skip(1) {

		let mut file =  match File::open(&arg) {
			Err(why) => panic!("couldn't open {}: {}", arg, why.description()),
			Ok(file) => file,
		};

		let mut s = String::new();
		match file.read_to_string(&mut s) {
			Err(why) => panic!("couldn't open {}: {}", arg, why.description()),
			_ => (),
		}

		let mut i = s.chars()
			.filter(|x| *x != '\n')
			.map(|x| Alphabet::try_from(x).unwrap());
		l.insert_words(&mut i, 5);
		println!("{}", serde_json::to_string(&l).unwrap());
	}
}

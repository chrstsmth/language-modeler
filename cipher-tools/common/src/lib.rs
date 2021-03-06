extern crate cipher_lib;
extern crate clap;

pub mod cli;
pub mod parse;

use cipher_lib::alphabet::latin::*;
use std::convert::TryFrom;

pub fn string_to_alph(s: String) -> Vec<Latin> {
	s.chars()
		.map(|c| Latin::try_from(c))
		.filter(|x| x.is_ok())
		.map(|x| x.unwrap())
		.collect()
}

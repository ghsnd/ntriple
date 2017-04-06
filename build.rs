/**
 * Copyright 2017 Gerald Haesendonck
 * Licensed under the Academic Free License version 3.0
 **/

extern crate peg;

fn main() {
	peg::cargo_build("grammar/ntriple.rustpeg");
}

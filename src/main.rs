#![allow(warnings)]


// PACKAGES
use std::io;


// MODULES
mod time;

mod init;
pub use crate::init::init_md;

mod blacklist;
pub use crate::blacklist::blacklist_md;


fn main()
{
	let args: Vec<_> = std::env::args().collect();

	if args.len() == 1 {
		println!("You should enter arguments to use the program.\n");
		io::stdin().read_line(&mut String::new()).unwrap();
		return;
	}

	let first_arg = std::env::args().nth(1).expect("no pattern given");

	match first_arg.as_ref()
	{
		"init" => {
			if args.len() < 2 {
				println!("You haven't written enough arguments.\nShould be -> mox init");
				return;
			}

		  init_md::start();
		},
		"add" => {},
		"del" => {},
		"get" => {},
		"log" => {},
		"info" => {},
		"exit" => return,
		_ => {
			println!("Incorrect command !");
			return;
		},
	}
}


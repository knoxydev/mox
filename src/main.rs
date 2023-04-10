#![allow(warnings)]

// PACKAGES
use std::io;

// MODULES



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
		  let db_name = std::env::args().nth(2).expect("error database name");
		  init_md::start(db_name.to_string());
		},
		"add" => add_md::start(),
		"delete" => {},
		"select" => {},
		"print" => {},
		"log" => {},
		"info" => {},
		"cmd" => {},
		"exit" => return,
		_ => {
			println!("Incorrect command !");
			return;
		},
	}
}

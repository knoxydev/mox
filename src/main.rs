#![allow(warnings)]


// PACKAGES
use std::io;


// MODULES
mod init;
pub use crate::init::init_md;

mod add;
pub use crate::add::add_md;

mod del;
pub use crate::del::del_md;

mod time;


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
			if args.len() < 4 {
				println!("You haven't written enough arguments.\nShould be 2 -> mox init [database's name] [amount of columns]");
				return;
			}

		  let db_name = std::env::args().nth(2).expect("error: database name");
		  if db_name == "init" {
		  	println!("error name");
		  	return;
		  }

		  let db_columns = std::env::args().nth(3).expect("error: the number of columns in the database").parse::<i64>().unwrap();
		  if db_columns <= 0 || db_columns >= 11 {
		  	println!("error number");
		  	return;
		  }

		  init_md::start(db_name, db_columns);
		},
		"add" => add_md::start(args),
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

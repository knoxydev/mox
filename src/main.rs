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
		  let db_name = std::env::args().nth(2).expect("error database name");
		  if db_name == "init" {
		  	println!("error name");
		  	return;
		  }

		  init_md::start(db_name);
		},
		"add" => add_md::start(),
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

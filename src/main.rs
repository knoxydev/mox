#![allow(warnings)]


// PACKAGES
use std::io;


// MODULES
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
			if args.len() < 3 {
				println!("You haven't written enough arguments.\nShould be 1 -> mox init [settings_file_name.yaml]");
				return;
			}

      let settings_name = std::env::args().nth(2).expect("[ERROR]: settings name");

		  // START BLACKLIST'S FUNCTION FOR CHECKING
			/* if blacklist_md::start(&db_name) == true
			{
				println!("\n[ERROR]: This name is on the blacklist !");
				println!("сhoose a different name for the database !\n-------------\n");
				return;
			} */

		  init_md::start(settings_name);
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


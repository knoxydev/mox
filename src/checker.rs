// MOD FOR CHECKING IS DATABASE EXIST OR NOT
pub mod checker_md
{
	use std::fs;
	use std::fs::File;
	use std::io::{BufRead, BufReader};


	pub fn get_db_name(db_path: String) -> (bool, String)
	{
		let mut x = String::new();
		let mut db_name = String::new();

		match File::open(db_path) {
			Ok(file) =>
			{
				let reader = BufReader::new(file);
				let mut line_count = 0;
				let mut line_exist: bool = false;

				for line in reader.lines() {
					line_count += 1;

					if line_count == 3 {
						match line {
							Ok(text) => {
								line_exist = true;
								x = text.to_string();
								break;
							}, Err(e) => {
								eprintln!("error with reading init.txt: {}", e);
								return (false, "error: third line in init.txt not detected".to_string());
								break;
							}
						}
					}
				}

				if line_exist == false { return (false, "error: third line in init.txt not detected".to_string()); }
			}, Err(e) => {
				eprintln!("error opening init.txt: {}", e);
				return (false, "error: init.txt".to_string());
			}
		}

		return (true, x.replace("db_name: ", ""));
	}


	pub fn start() -> (bool, String)
	{
		let mut db_is_ok: bool = false;
		let mut db_name: (bool, String) = (false, "".to_string());


		if fs::metadata("mox/init.txt").is_ok()
		{
			db_name = get_db_name("mox/init.txt".to_string());

			println!("{:?}", db_name);

			if db_name.0 == true { db_is_ok = true; }
			else {
				println!("\n---\nfatal error !");
				println!("delete the 'mox' folder and initialize the database again !\n---\n");
			}
		} else {
			println!("\n---\nfatal error !");
			println!("delete the 'mox' folder and initialize the database again !\n---\n");
		}

		return db_name;
	}	
}
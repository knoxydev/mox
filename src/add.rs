pub mod add_md
{
	use std::fs::File;
	use std::io::{prelude::*, BufReader};


	fn get_db_columns() -> std::io::Result<i64>
	{
		let file = File::open("mox/init.txt")?;
		let reader = BufReader::new(file);
		let mut columns: i64 = 0;

		{
			// GET ALL DATA FROM INIT.TXT
			fn get_value(x: String) -> (String, String)
			{
				let mut point : bool = false;
				let mut one = String::new();
				let mut two = String::new();

				for i in x.chars() {
					if i != ':' { one.push(i); }
					else { break; }}

				for i in x.chars() {
					if i == ':' { point = true; }
					if point == true { two.push(i); }}

				two.remove(0);
				two.remove(0);

				return (one, two);
			}

			let mut info = Vec::new();
			for line in reader.lines() { info.push(get_value(line.unwrap())); }
			for i in info
				{ if i.0 == "db_columns" { columns = i.1.parse::<i64>().unwrap(); }}
		}

		Ok(columns)
	}

	pub fn start(args: Vec<String>)
	{
		println!("{:?}", args);

		let x = get_db_columns();
		println!("{:?}", x);
	}
}
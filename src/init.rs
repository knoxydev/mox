pub mod init_md
{
	use std::fs;
	use std::env;


	fn add_info(db_name: String, db_columns: i64)
	{
		let time_date: [String; 2] = crate::time::time_fn::start();

		let cwd = env::current_dir().unwrap();
		let folder: String = String::from(cwd.to_string_lossy());
		let os_name = String::from(env::consts::OS);
		let created_date = &time_date[0];
		let created_time = &time_date[1];

		let info = format!("name: {folder}\nos_name: {os_name}\ndb_name: {db_name}\ndb_columns: {db_columns}\ncreated_date: {created_date}\ncreated_time: {created_time}");
		fs::write("mox/init.txt", info).expect("Unable to write file");
	}


	pub fn start(db_name: String, db_columns: i64)
	{
		let DB_NAME_STR: String = format!("mox/{}.txt", &db_name);

    match fs::create_dir("mox")
    {
			Err(why) =>
			{
				let mut db_exist: bool = if fs::metadata("mox/init.txt").is_ok() && fs::metadata(DB_NAME_STR).is_ok() { true }
    		else { false };

    		if db_exist == true
    			{ println!("database didn't created: {:?} !", why.kind()); }
    		else {
    			println!("\n---\nthe mox/init.txt and mox/db_name.txt are missing");
    			println!("delete the 'mox' folder and initialize the database again !\n---\n");
    		}

    		return;
			},
			Ok(_) =>
			{
				fs::File::create(DB_NAME_STR);

				match fs::File::create("mox/init.txt")
		    {
					Err(why) => println!("{:?} !", why.kind()),
					Ok(_) => add_info(db_name, db_columns),
				}		
			},
		}

		println!("Initialized !");
	}
}
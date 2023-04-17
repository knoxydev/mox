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
			Err(why) => println!("database didn't created: {:?} !", why.kind()),
			Ok(_) => {
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
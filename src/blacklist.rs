pub mod blacklist_md
{
	const names: &'static [&str] = &[
		"init", "git", "con", "prn",
		"aux", "nul", "mox",
		"/", "?", "|", "!", ":", "<", ">",
		"&", "*", "+", "=", "-", "_", ")",
		"(", "^", "%", "$", ";", "#", "№",
		"@", "'", "`", "~", ",", ".",
		"0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
	];

	pub fn start(db_name: &String) -> bool
	{
		let x: bool = names.contains(&db_name.as_str());
		if x == true { return true; }
		else { return false; }
	}
}

pub mod init_md
{
  use std::{fs, io};
  use std::env;
  use std::io::Write;
  use std::path::Path;

  use serde::Deserialize;


  #[derive(Debug, Deserialize)]
  struct Config
  {
    db_name: String,
    columns: Vec<String>
  }


  fn folder_check() -> bool
  {
    let folder_path = "mox";

    if Path::new(folder_path).exists() { return true; }
    else
    {
      match fs::create_dir(folder_path)
      {
        Ok(_) => return true,
        Err(e) => {
          println!("[ERROR]: Failed to create main folder '{}': {}", folder_path, e);
          return false;
        }
      }
    }
  }


  pub fn start(settings_filename: String) -> Result<(), Box<dyn std::error::Error>>
  {
    if folder_check() == true
    {
      let full_settings_filename: String = "mox/".to_owned() + &settings_filename;

      if Path::new(&full_settings_filename).exists()
      {
        let yaml_content = fs::read_to_string(full_settings_filename)?;
        let config: Config = serde_yaml::from_str(&yaml_content)?;

        println!("{:?}", config);
      }
      else { println!("[ERROR]: File '{}' doesn't exist.", full_settings_filename); }
    }
    else {  println!("[ERROR]: Failed to create main folder."); }


    Ok(())
  }
}

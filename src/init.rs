pub mod init_md
{
  use std::{fs, io};
  use std::env;
  use std::io::Write;
  use std::path::Path;

  use serde::{Serialize, Deserialize};


  #[derive(Debug, Serialize, Deserialize)]
  struct Config
  {
    db_name: String,
    columns: Vec<String>
  }


  fn create_block(db_config: Config) -> Result<(), Box<dyn std::error::Error>>
  {
    let db_name: String = format!("mox/{}.txt", db_config.db_name);
    let file = fs::File::create(&db_name)?;

    match fs::create_dir("mox/blocks/")
    {
      Ok(_) =>
      {
        let new_block_path: String = format!("mox/blocks/{}-block-1.txt", db_config.db_name);

        match fs::File::create(&new_block_path)
        {
          Ok(_) => {
            let time: [String; 2] = crate::time::time_fn::start();
            let line: String = format!("1|||{}|||{}\n", time[1], time[0]);

            let mut file = fs::File::create(db_name)?;
            file.write_all(line.as_bytes())?;

            println!("[SUCCESS]: the database was successfully created!");
          },
          Err(e) => println!("[ERROR]: Failed to create 'block': {}", e),
        }
      },
      Err(e) => println!("[ERROR]: Failed to create 'blocks' folder: {}", e),
    }

    Ok(())
  }


  fn check_folder() -> bool
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


  pub fn start() -> Result<(), Box<dyn std::error::Error>>
  {
    if check_folder() == true
    {
      if Path::new("mox/db-config.yaml").exists()
      {
        let yaml_content = fs::read_to_string("mox/db-config.yaml")?;
        let config: Config = serde_yaml::from_str(&yaml_content)?;

        if config.db_name.len() == 0 && config.columns.len() == 0
          { println!("[ERROR]: Fill in the configuration file (mox/config.yaml) with information about your database."); }
        else
          { create_block(config); }
      }
      else {
        let config = Config {
          db_name : "".to_string(),
          columns : [].to_vec(),
        };

        let file = fs::File::create("mox/db-config.yaml")?;
        serde_yaml::to_writer(file, &config)?;

        println!("[SUCCESS]: The foundation for the database has been created!\nFill in the 'mox/db-config.yaml' data!")
      }
    }

    Ok(())
  }


  /* pub fn start(settings_filename: String) -> Result<(), Box<dyn std::error::Error>>
  {
    if check_folder() == true
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
  } */
}






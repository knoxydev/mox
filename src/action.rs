pub mod action_md
{
  use std::fs;
  use serde::{Serialize, Deserialize};


  #[derive(Debug, Serialize, Deserialize)]
  struct Actions_Tree
  {
    db_columns: String,
    order: Vec<String>,
    add: Vec<String>,
    del: Vec<String>,
    get: String
  }


  pub fn start(db_columns: Vec<String>)
  {
    match fs::create_dir("mox/actions/")
    {
      Ok(_) => {
        // CREATING STRING WITH DB-COLUMNS BY USING VECTOR<STR> (DB_COLUMNS)
        let mut column_names: String = String::new();
        for (idx, val) in db_columns.iter().enumerate() {
          if idx == db_columns.len() - 1 { column_names += val; }
          else { column_names += &format!("{} | ", val).to_string(); }
        }

        let actions_tree = Actions_Tree
        {
          db_columns : column_names,
          order : Vec::new(),
          add : Vec::new(),
          del : Vec::new(),
          get : String::new()
        };

        match fs::File::create("mox/actions/actions.yaml")
        {
          Ok(file) => {
            match serde_yaml::to_writer(file, &actions_tree) {
              Ok(_) => println!("[SUCCESS]: 'mox/actions/actions.yaml' was created successfully. Now you can interact with the database!\n"),
              Err(e) => println!("[ERROR]: Failed to save data to 'mox/actions/actions.yaml' file: {}", e),
            }
          },
          Err(e) => println!("[ERROR]: Failed to create 'mox/actions/actions.yaml' file: {}", e),
        }

      },
      Err(e) => println!("[ERROR]: Failed to create 'actions' folder: {}", e),
    }
  }
}




extern crate serde_json;

use std::env;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::{Write};
use std::os::unix::fs::PermissionsExt;
use log::{debug};
use std::collections::HashMap;

fn main() -> std::io::Result<()>  {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    // handling of args
    match args.len() {
      // no arguments passed
      1 => {
          println!("My name is 'match_args'. Try passing some arguments!");
      },
      4 => {
          let jsonenv_arg = &args[2];
          let scriptenv_arg = &args[3];
        
          let path_json_file = Path::new(jsonenv_arg);    
          let path_examplesh = Path::new(scriptenv_arg);
          let display = path_examplesh.display();
          // variables for handling json's file  
          let file = fs::File::open(path_json_file).expect("file should open read only");
          let json: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");
        
          // Open a file in write-only mode, returns `io::Result<File>`
          let file_examplesh = match File::create(&path_examplesh) {
              Err(why) => panic!("couldn't create {}: {}", display, why),
              Ok(file) => file,
          };
          // Set script's permissions
          let mut perms = fs::metadata(path_examplesh)?.permissions();
          perms.set_mode(0o777); 
          fs::set_permissions(path_examplesh, perms)?;
    
          let cmd = &args[1];
          // parse the command
          match &cmd[..] {
              "settings_local" => create_settings_local(json, file_examplesh, display),
              "sh_envs" => create_shscript_env(json, file_examplesh, display),
              _ => {
                  eprintln!("error: invalid command");
                  help();
              },
          }
      },
        // all the other cases
        _ => {
          // show a help message
          help();
      }
    }

    Ok(())   
}

// here we will generate sh scripts with envs
fn create_shscript_env(json: serde_json::Value, mut file_examplesh: File, display: std::path::Display){
  for (env_key, env_val) in json.as_object().unwrap().iter() {
    debug!("SetingUp -- export {}={}", env_key, env_val);

    // Write the `env_key and env_val` string to `file_examplesh`, returns `io::Result<()>`
    match writeln!(&mut file_examplesh, "export {}={}", env_key, env_val) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => debug!("successfully wrote to {}", display),
    }
  }
}

fn help() {
  println!("usage:
make_envs {{settings_local|sh_envs}} <string>
          {{path to file name with secrets}} <string>
          {{path to file name which will be generated }} <string>");
}

// here we will generate settings_local.py
fn create_settings_local(json: serde_json::Value, mut file_examplesh: File, display: std::path::Display){
  let delimeter_title = "_";
  let delimeter_var = "__";
  let mut var_dict: HashMap<String, Vec<String>> = HashMap::new();    
  for (env_key, env_val) in json.as_object().unwrap().iter() {      
      debug!("SetingUp -- {}={}", env_key, env_val);
      let (_,var_not_title) = env_key.split_once(delimeter_title).unwrap();        
      let (key,variable) = cut_variable(var_not_title,delimeter_var);        
      dict_variable(&mut var_dict,key,variable,&env_val.to_string());
      if key.trim().is_empty() {
        match writeln!(&mut file_examplesh, "{}={}", variable,env_val.to_string()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => debug!("successfully wrote to {}", display),
        }
      }        
  }
  for (key, value) in &var_dict {
    let mut subvar_dict: HashMap<String, Vec<String>> = HashMap::new();
    for subvar in value.iter() {
      let (res_key,res_val) = subvar.split_once(delimeter_var).unwrap();
      if !res_key.is_empty() { 
        match subvar_dict.get_mut(res_key) {
          Some(list) => list.push(res_val.to_string()),
          None => {
              let vector_var = vec![res_val.to_string()];
              subvar_dict.insert(res_key.to_string(), vector_var);
          }
        }    
      }       
    }
    for (sub_key, sub_value) in subvar_dict.into_iter() {
      let mut sum_sub_value = "".to_string();        
      for elem in sub_value.into_iter() {
        let tmp = format!("\"{}", elem);
        sum_sub_value.push_str(&tmp);
      }
      debug!("{}={{{}\":{{{}}}}}", key, sub_key.trim(), sum_sub_value);
      match writeln!(&mut file_examplesh, "{}={{{}\":{{{}}}}}", key, sub_key.trim(), sum_sub_value) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => debug!("successfully wrote to {}", display),
      }
    }
  }
}

// here we will cut varible with delimeter  
fn cut_variable<'a>(var_not_title: &'a str, delimeter_var: &'a str) -> (&'a str, &'a str)  {   
   debug!("{}", var_not_title);   
   if var_not_title.contains(delimeter_var) {
     let (part1,part2) = var_not_title.split_once(delimeter_var).unwrap();     
     return (part1,part2)
   } else {
     let (part1,part2) = ("",var_not_title);
     return (part1,part2)
   } 
}

// here we will generate a dictionary for settings_local.py
fn dict_variable<'a>(dict: &mut HashMap<String,Vec<String>>, key: &'a str, variable: &'a str, env_val: &'a str) {  
  if !key.is_empty() { 
    let env_val_format = format!("\"{}\":{},",variable.to_string(), env_val);
    match dict.get_mut(key) {
      Some(list) => list.push(env_val_format),
      None => {
          let vector_var = vec![env_val_format];
          dict.insert(key.to_string(), vector_var);
      }
    }    
  }  
}
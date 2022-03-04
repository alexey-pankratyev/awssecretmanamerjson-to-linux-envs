extern crate serde_json;

use std::env;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::{Write};
use std::os::unix::fs::PermissionsExt;

fn main() -> std::io::Result<()>  {

    let args: Vec<String> = env::args().collect();
    let jsonenv_arg = &args[1];
    let scriptenv_arg = &args[2];

    let path_json_file = Path::new(jsonenv_arg);    
    let path_examplesh = Path::new(scriptenv_arg);
    let display = path_examplesh.display();

    let file = fs::File::open(path_json_file).expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file_examplesh = match File::create(&path_examplesh) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Set script's permissions
    let mut perms = fs::metadata(path_examplesh)?.permissions();
    perms.set_mode(0o777); 
    fs::set_permissions(path_examplesh, perms)?; 
    
    for (env_key, env_val) in json.as_object().unwrap().iter() {
        println! ("SetingUp -- export {}={}", env_key, env_val);

        // Write the `env_key and env_val` string to `file_examplesh`, returns `io::Result<()>`
        match writeln!(&mut file_examplesh, "export {}={}", env_key, env_val) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    Ok(())
    
}

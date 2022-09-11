use std::*; // si
use std::io::Write;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct JsonTemplate {
    keywords: Vec<(String, String)>,
    textfile: String
}

fn get_username() -> Result<String, io::Error> {
    let output = process::Command::new("whoami").output();
    match output {
        Ok(cmdresult) => {
            let mut username = String::from_utf8(cmdresult.stdout).unwrap();
            username.pop();
            Ok(username)
        }
        Err(err) => Err(err)
    }
}

fn read_string() -> Result<String, io::Error> {
    let mut s = String::new();
    let res = io::stdin().read_line(&mut s);
    match res {
        Ok(_) => Ok(s),
        Err(err) => Err(err)
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    print!("Template name: ");
    io::stdout().flush()?;
    let name = read_string().unwrap();
    let name = name.trim();

    let username = get_username()?;
    let json_path = "/home/".to_owned() + &username + "/.config/newmakefile/" + &name + ".json";
    if !path::Path::new(&json_path).is_file() {
        eprintln!("Json file not found in ~/.config/newmakefile");
        return Ok(())
    }

    let template = match serde_json::from_str::<JsonTemplate>(str::from_utf8(&(fs::read(&json_path)?))?) {
        Ok(json) => json,
        Err(_) => {
            eprintln!("Json malformed");
            return Ok(())
        }
    };

    let text_path = "/home/".to_owned() + &username + "/.config/newmakefile/" + &template.textfile;
    let text = match fs::read(&text_path) {
        Ok(vec) => String::from_utf8(vec)?,
        Err(_) => {
            eprintln!("Error opening text file");
            return Ok(())
        }
    };

    let mut formatted_text = text.clone();
    for (output, key) in template.keywords {
        print!("{}", &output);
        io::stdout().flush()?;
        formatted_text = formatted_text.replace(&key, read_string()?.trim());
    }

    match fs::write("./Makefile", &formatted_text) {
        Ok(()) => println!("Makefile successfully created"),
        Err(_) => {
            eprintln!("Error writing formatted text into Makefile");
            return Ok(())
        }
    }

    Ok(())
}

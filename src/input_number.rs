use std::io::{self, Write};
use crate::color_func;

fn input_string() -> String{
    let mut  input_string:String = String::new();

    print!("Input U number : ");
    std::io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input_string) {
        Ok(_) => input_string,
        Err(_) => "nil".to_string(),
    }
}

pub  fn input_number() -> i8{
    loop {
        let num_string: String = input_string();
        match num_string.trim().parse::<i8>() {
            Ok(num) => {
                println!("\tnumber : ✅\n");
                return num;
            },
            Err(e) => {
                color_func::is_err(e.to_string().as_str());
                continue;                
            },
        }
    }
}
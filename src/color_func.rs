use colored::*;

pub fn is_err(text: &str) {
    let error_text = "Error".red();
    let text_out = text.yellow();

    println!("{} : {}",error_text,text_out);
}

pub fn is_show(text: &str) {
    let text_out = text.cyan();
    print!("{}",text_out);
}
mod input_number;
mod color_func;
mod func_show;

use input_number::input_number;
use color_func::is_err;
use func_show::pyramid;
use func_show::inverted_pyramid;
use func_show::diamond;

fn main() {

    loop {
        println!("This Is make
         == 1 : make pyramid
         == 2 : make inverted_pyramid
         == 3 : make diamond
         => 4 : Exit "
        );
        
        let mut num: i8 = input_number();
        let check_type: i8 = match num {
            1 | 2 | 3 => num.clone(),
            _ => {
                println!("End Process :)");
                break;
            },
        };

        println!("limit 1 - 9");
        num = input_number();
        let check_limit: i8 = match num {
            0..10 => num.clone(),
            _ => {
                is_err("Get Number limit");
                continue;  
            },
        };
        
        match check_type {
            1 => pyramid(check_limit),
            2 => inverted_pyramid(check_limit),
            3 => diamond(check_limit),
            _ => {
                is_err("Idk why error here");
                continue;
            },
        }
    }
}


use std::io::Write;

use crate::color_func::is_show;

pub fn pyramid(limit: i8) {
    top_to_down(limit);
}

pub fn inverted_pyramid(limit: i8) {
    down_to_top(limit);
}

pub fn diamond(limit: i8) {
    top_to_down(limit -1);
    fix_diamond(limit);
    down_to_top(limit -1);
}

fn top_to_down(limit: i8){
    let mut start: i8 = 1;
    let mut min_line: i8 = limit;
    std::io::stdout().flush().unwrap();

    for _ in 0..limit{
        
        for _ in 0..min_line{
            print!("  ");
        }

        for _ in 0..start{
            is_show("* ");
        }

        for _ in 0..start-1{
            is_show("* ");
        }

        start += 1;
        min_line -= 1;
        println!("");
    }
}

fn down_to_top(limit: i8){
    let mut start: i8 = limit;
    let mut min_line: i8 = 1;
    std::io::stdout().flush().unwrap();

    for _ in 0..limit{
        
        for _ in 0..min_line{
            print!("  ");
        }

        for _ in 0..start{
            is_show("* ");
        }

        for _ in 0..start-1{
            is_show("* ");
        }

        start -= 1;
        min_line += 1;
        println!("");
    }
}

fn fix_diamond(limit: i8){
    for _ in 0..(limit*2)-1 {
        is_show("* ");
    }
    println!()
}
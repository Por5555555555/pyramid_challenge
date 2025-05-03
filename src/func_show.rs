use crate::color_func::is_show;

pub fn pyramid(limit: i8) {
    let limit: i8 = limit;
    let min_line: i8 = limit;
    let num_add: i8 = 1;
    let num_start: i8 = 1;
    
    show_func(limit, min_line, num_add, num_start);
}

pub fn inverted_pyramid(limit: i8) {
    let limit: i8 = limit;
    let min_line: i8 = 1;
    let num_add: i8 = -1;
    let num_start: i8 = limit;

    show_func(limit, min_line, num_add, num_start);
}

pub fn diamond(limit: i8) {
    let fix = 1+limit*2;
    
    pyramid(limit);
    fix_diamond(fix);
    inverted_pyramid(limit);
}

fn show_func(limit: i8,min_line: i8, num_add: i8,num_start: i8){
    let num_add: i8 = num_add;
    let mut start: i8 = num_start;
    let mut min_line: i8 = min_line;

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

        start += num_add;
        min_line -= num_add;
        println!("");
    }
}

fn fix_diamond(fix: i8){
    for _ in 0..fix {
        is_show("* ");
    }
    println!()
}
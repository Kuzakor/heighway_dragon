//Made by Kuzakor 

#![allow(unused)]
//Imports
use std::io;
use turtle::Turtle;

fn main() {
    //Asking user for input
    println!("How many iterations? 1 iteration == 4 lines: ");
    let n = get_int();
    println!("Which way? (right-0, left-1): ");
    let w = get_int();
    println!("Generating........."); 

    //Generating layout to fill: [102102102102102102102]
    // 2 - left
    // 1 - right
    let mut vec1: Vec<u8> = Vec::new();
    for _i in 0..n {
        match w {
            0 => {
                vec1.push(1);
                vec1.push(0);
                vec1.push(2);
                vec1.push(0);
            }
            1 => {
                vec1.push(2);
                vec1.push(0);
                vec1.push(1);
                vec1.push(0);
            }
            _ => {
                println!("Wrong number");
                break;
            }
        }
        
    }

    //Filling layout 
    for i in 0..vec1.len() / 2  {
        match i {
                0 => vec1[1] = vec1[i],
                _ => vec1[i*2+1] = vec1[i],
        }
    }

    //Asking user for input
    println!("Enter lenght of a single line: ");
    let lenght = get_int();
    println!("Drawing.....");

    //Setting up turtle
    let mut turtle = Turtle::new();
    turtle.hide();
    turtle.set_speed("instant");

    //Drawing
    for i in &vec1{
        turtle.forward(lenght as f64);
        match i {
            1 => turtle.right(90.0),
            2 => turtle.left(90.0),
            _ => println!("Error in generating")
        }
    }

}

//Function getting u32 input form user
fn get_int() -> u32 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("read error");
    let num:u32 = num.trim().parse().expect("convert error");
    return num;
}

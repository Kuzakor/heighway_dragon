//Made by Kuzakor
//Imports
use std::io;
use turtle::Turtle;

//new type of variable - instruction
#[derive(Copy, Clone, Debug)]
enum Instruction {
    Left,
    Right,
    Empty,
}

fn main() {
    //Asking user for input
    let n = {
        println!("How many iterations? 1 iteration == 4 lines: ");
        get_int()
    };

    let which_way = {
        println!("Which way? (right-0, left-1): ");
        get_int()
    };

    //Generating layout to fill: [Right, Empty, Left, Empty ................]
    println!("Generating.........");
    let mut layout: Vec<Instruction> = Vec::new();
    for _ in 0..n {
        match which_way {
            0 => {
                layout.push(Instruction::Right);
                layout.push(Instruction::Empty);
                layout.push(Instruction::Left);
                layout.push(Instruction::Empty);
            }
            1 => {
                layout.push(Instruction::Left);
                layout.push(Instruction::Empty);
                layout.push(Instruction::Right);
                layout.push(Instruction::Empty);
            }
            _ => {
                panic!("Wrong direction number");
            }
        }
    }

    //Filling layout by replacing Empty with Right or Left by setting the index 2 times bigger to the same value
    for i in 0..layout.len() / 2 {
        match i {
            0 => layout[1] = layout[0],
            _ => layout[i * 2 + 1] = layout[i],
        }
    }

    //Asking user for input
    let lenght = {
        println!("Enter lenght of a single line: ");
        get_int()
    };

    //Setting up turtle
    let mut turtle = Turtle::new();
    turtle.hide();
    turtle.set_speed("instant");

    //Drawing
    println!("Drawing.....");
    for i in &layout {
        turtle.forward(lenght as f64);
        match i {
            Instruction::Right => turtle.right(90.0),
            Instruction::Left => turtle.left(90.0),
            _ => panic!("Error in generating layout"),
        }
    }
    println!("The dragon is finished!");
}

//Function getting u32 input form user
fn get_int() -> i32 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("read error");
    let num: i32 = num.trim().parse().expect("convert error");
    num
}

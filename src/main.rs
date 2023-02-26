#![allow(unused)]
use std::io;
use turtle::Turtle;


 fn main() {
    let mut turtle = Turtle::new();
    println!("Podaj ilosc: ");
    let n = get_int();
    println!("Generowanie........."); 
    let mut vec1: Vec<char> = Vec::new();
    for _i in 0..n {
        vec1.push('R');
        vec1.push(' ');
        vec1.push('L');
        vec1.push(' ');
    }

    for i in 0..vec1.len() {
        if i < vec1.len() / 2 {
            if i == 0 {
                vec1[1] = vec1[i];
            } else {
                vec1[i*2+1] = vec1[i];
            }
        
        }
    }
    println!("Podaj dlugosc lini: ");
    let lenght = get_int();
    println!("Rysowanie.....");

    for i in &vec1{
        turtle.forward(3.0);
        match i {
            'R' => turtle.right(90.0),
            'L' => turtle.left(90.0),
            _ => println!("Koniec")
        }
    }

}

fn get_int() -> u32 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("select");
    let num:u32 = num.trim().parse().expect("select");
    return num;
}
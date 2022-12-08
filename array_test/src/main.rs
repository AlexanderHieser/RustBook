use std::{io, process::exit};
fn main() {


    let a = [1, 2, 3, 4, 5];

    println!("Current Array: {:?}", a);
    println!("Please provide an index (starting with 0");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please insert valid index number");
            exit(-1);
        }
    };
    let element = a[index];
    println!("Element Value: {element}");
}

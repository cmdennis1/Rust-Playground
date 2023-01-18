use std::io;

fn main() {
    let array = [10, 11, 12, 13, 14, 15, 16];
    let mut index = String::new();
    loop {
        println!("Enter an index:");

        io::stdin()
            .read_line(&mut index)
            .expect("Error reading input");
        let index: usize = match index.trim().parse() {
            Ok(number) => {
                if number < array.len() {
                    number
                } else {
                    println!("Index out of bounds, please try again.");
                    index.clear();
                    continue;
                }
            },
            Err(_) => {
                println!("Please enter a number!");
                index.clear();
                continue;
            }
        };

        let element = array[index];
        println!("The value of the element at index {} is {}", index, element);
        break;
    }
}
  



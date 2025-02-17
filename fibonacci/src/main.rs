use std::io;
fn main() {
    println!("Enter the nth digit of the Fibonacci series!");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Failed to read line!");
    let mut y: i32 = y.trim().parse().expect("Failed to convert value!");
    let mut prev = 0;
    let mut next = 1;
    if y == 1 {
        println!("{prev}");
    } else if y <= 0 {
        println!("Enter only positive values!");
    } else {
        while y > 1 {
            let temp = next;
            next = prev + next;
            prev = temp;
            y -= 1;
        }
        println!("The nth fibonacci is {prev}");
    }
}

use std::io;

fn main() {
    let repeat = "=".repeat(30);
    println!("{repeat}");
    println!("Rust Calculator");
    println!("{repeat}");
    
    loop {
        let mut operation = String::new();
        let mut a = String::new();
        let mut b = String::new();

        println!("\n[1] Add\n[2] Subtraction\n[3] Multiplication\n[4] Division\n[5] Power");
        println!("What is your desired operation?");

        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

        let operation: u32 = match operation.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("First Number:");
        io::stdin().read_line(&mut a).expect("Failed to read line");
        let first_num: u32 = a.trim().parse().expect("Invalid input");

        println!("Second Number");
        io::stdin().read_line(&mut b).expect("Failed to read line");
        let second_num: u32 = b.trim().parse().expect("Invalid input");

        if operation == 1 {
            println!("This addition is equal to {}",first_num+second_num)
        } else if operation == 2 {
            println!("This subtraction is equal to {}",first_num-second_num)
        } else if operation == 3 {
            println!("This multiplication is equal to {}",first_num*second_num)
        } else if operation == 4 {
            println!("This division is equal to {}",first_num/second_num)
        } else {
            println!("This power is equal to {}",first_num.pow(second_num))
        }
    }
}

use std::io;

fn main() {
    let mut user_input = String::new();
    println!("\nEnter your Fibo Number: ");

    io::stdin().read_line(&mut user_input).expect("Error Occured!");
    let input_int = user_input.trim().parse::<u32>().unwrap();

    let mut n1: u32 = 0;
    let mut n2: u32 = 1;
    let mut next_term: u32;

    let mut index: u32 = 0;

    
    while index < input_int {
        index = index + 1;
        println!("{}", n1);

        next_term = n1 + n2;
        n1 = n2;
        n2 = next_term;        
    }
    
}

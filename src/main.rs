fn main() {
    let mut n1: u32 = 0;
    let mut n2: u32 = 1;
    let mut next_term: u32;

    let mut index: u32 = 0;

    
    while index < 21 {
        index = index + 1;
        println!("{}", n1);

        next_term = n1 + n2;
        n1 = n2;
        n2 = next_term;        
    }
    
}

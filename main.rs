use std::io;

fn number_to_vec(number: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    let mut n = number;
    
    while n > 9 {
        digits.push(n % 10);
        n = n / 10
    }
    
    digits.push(n);
    digits.reverse();
    
    digits
}

fn is_an_armstrong_number(number: i32) -> bool {
    let number_vec: Vec<i32> = number_to_vec(number);
    let mut sum: i32 = 0;
    
    for i in 0..number_vec.len(){
        sum += number_vec[i].pow(number_vec.len() as u32)
    }
    
    let result: bool = if sum == number && number > 0 {
        true
    } else {
        false
    };
    
    result
}

fn main() {
    loop {
        println!("Please input the number to check if it is an Armstrong number:");
    
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)  
            .expect("Failed to read line");
            
        let number = input.trim().parse::<i32>().expect("Invalid input");
            
        let is_an_armstrong_number: bool = is_an_armstrong_number(number);
        
        if is_an_armstrong_number {
            println!("{} is an Armstrong number", number)
        } else {
            println!("{} is not an Armstrong number", number)
        }
    }
}

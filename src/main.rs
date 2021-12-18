use std::io;

fn main() {
    println!("Welcome to get your fibonacci number."); 

    loop {
        let mut their_number:String = String::new();
        println!("How many fibonacci numbers would you like?");
        io::stdin()
            .read_line(&mut their_number)
            .expect("Failed to read line");

        //then parse it

        let their_number:String = match their_number.trim().parse() {
            Ok(response) => response,
            Err(_) => "ERROR".to_string()

        };
        //then do what with it,
        //print the fibonacci sequence

        println!("Your Fibonacci numbers are: {:?}", get_fibonacci_numbers(their_number.parse::<i32>().unwrap()));

    }
    
}

fn get_fibonacci_numbers(n:i32) -> Vec<i32> {
    let mut fibo_numbers:Vec<i32> = Vec::new();
    
    for element in 0..n {
        if element == 0 {
            fibo_numbers.push(0);
            continue
        }
        else if element == 1 {
            fibo_numbers.push(1);
        }
        else {
            
            fibo_numbers.push(fibo_numbers[element as usize - 1] + fibo_numbers[element as usize - 2]);
        }
    }

    return fibo_numbers
}
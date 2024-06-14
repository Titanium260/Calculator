use std::io::stdin;

fn main() {
    
    println!("Welcome to calculator!");
    
    let mut m1 = String::new();
    let mut v1 = String::new();
    let mut v2 = String::new();

    println!("Please select work mode (number) :\n");
    println!("1 - Addition"                      );
    println!("2 - Multiplication"                );
    println!("3 - Subtraction"                   );
    println!("4 - Division\n"                    );

        stdin()
            .read_line(&mut m1)
            .expect("failed to read from stdin");
            

    println!("Please enter first value..." );   

        stdin()
            .read_line(&mut v1)
            .expect("failed to read from stdin"  );

    println!("Please enter second value...");
    
        
        stdin()
            .read_line(&mut v2)
            .expect("failed to read from stdin"  );

    let mode:       i32 = m1.trim().parse().expect("Input not an integer");
    let x:          i32 = v1.trim().parse().expect("Input not an integer");
    let y:          i32 = v2.trim().parse().expect("Input not an integer");
    let mut result: i32 = 0;

        match mode {
            1 => *&mut result = x + y,
            2 => *&mut result = x * y,
            3 => *&mut result = x - y,
            4 => *&mut result = x / y,
            _ => println!("Can't you f*cking read!?")
        }

    println!("Result: {}", result);
 
}

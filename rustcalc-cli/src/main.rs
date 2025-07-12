use std::io;
// here io::stdin() - gives access to user input
// read_line reads what user has typed

// &mut taking the user input and store in it the variable

//unwrap means if there's an error just crash whole code

// use of trim () When you type into the terminal and press Enter, your input becomes something like "add\n" or "exit\n"

// If you don’t trim(), your program checks for "exit" and doesn't match, because it’s "exit\n" with a hidden newline.
// using shadowing — that means we’re reusing the same variable name, but replacing its value with a new one.

fn add (x:i32,y:i32) -> i32 { x + y}
fn sub (x:i32,y:i32) -> i32 { x - y}
fn mul (x:i32,y:i32) -> i32 { x * y}
fn div (x:i32,y:i32) -> i32 { x / y}

fn main() {
    loop {
        println!("Enter operation (add ,sub, mul, div) or 'exit' to quit:");

        let mut operation = String::new();
        io::stdin().read_line(&mut operation).unwrap();
        let operation = operation.trim();
        if operation == "exit" {
            break;
        }

        println!("Enter the first number :");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).unwrap();
        let num1 : i32 = num1.trim().parse().unwrap();

        println!("Enter the second number :");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).unwrap();
        let num2 :i32 = num2.trim().parse().unwrap();
        if operation == "add" {
           println!("Result :{}",add(num1,num2)); 
        } else if operation == "sub"{
          println!("Result :{}",sub(num1,num2)); 
        }
         else if operation == "mul"{
          println!("Result :{}",mul(num1,num2)); 
        }
        else if operation == "div"{
          println!("Result :{}",div(num1,num2)); 
        } else {
            println!("unknown operation");
        }

}
}



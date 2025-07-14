use std::io;

fn main() {
    let mut task1 = String::new();
    let mut task2 = String::new();
    let mut task3 = String::new();

    loop {
        println!("\n--- TASK MASTER ---");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Exit CLI");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        if choice.trim() == "1" {
            if task1.is_empty() {
                println!("Enter first task:");
                io::stdin().read_line(&mut task1).unwrap();
                println!("Task 1 added.");
            } else if task2.is_empty() {
                println!("Enter second task:");
                io::stdin().read_line(&mut task2).unwrap();
                println!("Task 2 added.");
            } else if task3.is_empty() {
                println!("Enter third task:");
                io::stdin().read_line(&mut task3).unwrap();
                println!("Task 3 added.");
            } else {
                println!("You can only add 3 tasks. Restart to reset.");
            }

            
        } else if choice.trim() == "2" {
            println!("\n--- Your Tasks ---");
            if !task1.is_empty() {
                println!("1. {}", task1.trim());
            }
            if !task2.is_empty() {
                println!("2. {}", task2.trim());
            }
            if !task3.is_empty() {
                println!("3. {}", task3.trim());
            }
            if task1.is_empty() && task2.is_empty() && task3.is_empty() {
                println!("(No tasks yet)");
            }
        } else if choice.trim() == "3" {
            println!("Exiting TaskMaster.");
            break;
        } else {
            println!("Invalid input. Try again");
        }
    }
}

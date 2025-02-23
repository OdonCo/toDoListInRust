use std::io;

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    println!("Tasks:");

    if tasks.is_empty(){
        println!("You currently don't have any task please add a task!");
        println!("1.Add task 2.Quit");

        let mut user_choice = String::new();

        io::stdin()
            .read_line(&mut user_choice)
            .expect("failed to read from stdin");

        let trimmed_user = user_choice.trim();

        if trimmed_user == "1"{
            add_task();
        } else if trimmed_user == "2"{
            return;
        }else{
            println!("Invalid input you can only enter 1 or 2");
        }

    }else{
        for task in &tasks{
            println!("{}", task);
        }
    }
    
    
}

fn add_task() -> String{
    println!("Enter the task you wanna add: ");

    let mut add_task = String::new();

    io::stdin()
        .read_line(&mut add_task)
        .expect("failed to read from stdin");

    let trimmed_task = add_task.trim().to_string();
    println!("adding: {}",trimmed_task);
    trimmed_task
}


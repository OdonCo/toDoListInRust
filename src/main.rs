// to the person who is reading this:
// i dont recommend using this code or trying to learn from it especially if your beginner since im also beginner
// and thats it i think

use std::{io, usize};


fn main() {
    let mut tasks: Vec<String> = Vec::new();

    while true{

    println!("You have total of: {} task ", tasks.len());

    if tasks.len() == 0{
        println!("You currently don't have any task please add a task!");
        println!("1.Add task 2.Quit");

        let mut user_choice = String::new();

        io::stdin()
            .read_line(&mut user_choice)
            .expect("failed to read from stdin");

        let trimmed_user = user_choice.trim();

        if trimmed_user == "1"{
            tasks.push(add_task());
        } else if trimmed_user == "2"{
            break;
        }else{
            println!("Invalid input you can only enter 1 or 2");
        }
    } else{
        for (index, task) in tasks.iter().enumerate() {
            println!("{}. {}", index + 1, task);
        }
        println!();
        println!("1.Add task");
        println!("2.Remove task");
        println!("3.Quit");

        let mut main_functions = String::new();

        io::stdin()
            .read_line(&mut main_functions)
            .expect("Failed to read message");
        let trim_mainf = main_functions.trim();

        if trim_mainf == "1"{
            tasks.push(add_task());
        }else if trim_mainf == "2"{
            remove_task(&mut tasks);
        } else if trim_mainf == "3"{
            break;
        } else{
            println!("Invalid input");
        }
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

fn remove_task(tasks: &mut Vec<String>){
    if tasks.is_empty() {
        println!("No tasks to remove.");
        return;
    } println!("Select a task to remove:");
    for (index, task) in tasks.iter().enumerate() {
        println!("{}. {}", index + 1, task);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failuire");
    
    if let Ok(choice) = input.trim().parse::<usize>(){
        if choice > 0 && choice <= tasks.len(){
            let removed_task = tasks.remove(choice-1);
            println!("Removed: {}", removed_task);
        } else{
            println!("Invalid task number");
        }
    } else {
        println!("Invalid input. Please enter a number.");   
    }
}

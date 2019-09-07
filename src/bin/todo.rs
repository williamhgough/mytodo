extern crate mytodo;

use colored::Colorize;
use mytodo::db::{
    create_task, establish_connection, mark_complete, models, query_task, remove_task,
};
use std::env;

fn help() {
    println!("subcommands:");
    println!("    new<title>: create a new task");
    println!("    show: show all tasks");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        "complete" => complete_task(&args[2..]),
        "delete" => delete_task(&args[2..]),
        _ => help(),
    }
}

fn new_task(args: &[String]) {
    if args.is_empty() {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    create_task(&conn, &args[0]);
}

fn show_tasks(args: &[String]) {
    if !args.is_empty() {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    display_tasks(query_task(&conn));
}

fn complete_task(args: &[String]) {
    if args.is_empty() {
        println!("complete: missing <id>");
        help();
        return;
    }

    let task_id = &args[0].parse::<i32>().expect("failed to parse task id");

    let conn = establish_connection();
    mark_complete(&conn, *task_id);
    display_tasks(query_task(&conn));
}

fn delete_task(args: &[String]) {
    let conn = establish_connection();
    let task_id = &args[0].parse::<i32>().expect("failed to parse task id");

    remove_task(&conn, *task_id);
}

fn display_tasks(tasks: Vec<models::Task>) {
    match tasks.len() {
        0 => {
            println!("You have no tasks to do!\n");
            println!("You can add one like so: todo new 'be awesome'");
        }
        _ => {
            println!("TASKS\n-----");
            for task in tasks {
                if task.done {
                    println!("{}. {}", task.id, task.title.strikethrough().green());
                } else {
                    println!("{}. {}", task.id, task.title.yellow());
                }
            }
        }
    }
}

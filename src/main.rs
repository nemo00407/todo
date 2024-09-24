/*
to-do list command line application
function:
1. add : add some task to to-do list
2. done : mark the task that have done
3. sort : put already done task to the end of list and sort the list by input order
4. todo : list the current task in the list
*/

use std::fs::OpenOptions;
use std::io::{self, Read};
use std::io::{Write, Result};
use serde::{Serialize, Deserialize};
use serde_json;
use clap::{Arg, Command};


#[derive(Serialize, Deserialize, Debug)]
struct Task {
    taskname: String,
    description: String,
    deadline: String,
    status: bool,
}
impl Task {
    fn build_task(taskname: String, description: String, deadline: String, status: bool) -> Task {
        Task {
            taskname,
            description,
            deadline,
            status,
        }
    }
}


fn sort() -> Result<()>{
    let mut file = OpenOptions::new()
        .read(true)
        .open("tasks_list.json")?;
    let mut task_json = String::new();
    file.read_to_string(&mut task_json).unwrap();
    let task_list: Vec<Task> = serde_json::from_str(&task_json).unwrap();
    let mut cur = 1;
    for task in &task_list {
        if task.status == true {
            println!("{} {} => {}", cur, task.taskname, task.deadline);
            cur += 1;
        }
    }
    for task in task_list {
        if task.status == false {
            println!("{} {} => {} (done)", cur, task.taskname, task.deadline);
            cur += 1;
        }
    }
    Ok(())
}

fn add(task: &String) -> Result<()> {
    println!("Add description for {task}: ");

    let mut des: String = String::new();
    let _num: usize = io::stdin().read_line(&mut des).expect("Failed to read description");
    println!("Add deadline for {task}: ");
    let mut date: String = String::new();
    let _num: usize = io::stdin().read_line(&mut date).expect("Failed to read deadline");
    
    let t = Task::build_task(task.to_owned(), des.trim().to_owned(), date.trim().to_owned(), true);
    
    let mut file = OpenOptions::new()
        .write(true)  // 寫入
        .read(true)
        .create(true) // 如果不存在就開新的
        .open("tasks_list.json")?;

    let mut task_json = String::new();
    match file.read_to_string(&mut task_json) {
        Ok(_rel) => println!("Success to open json file."),
        Err(error) => println!("Problem with open json file: {error:?}"),
    }

    if !(task_json.trim().is_empty()) {
        let mut task_list: Vec<Task> = serde_json::from_str(&task_json).unwrap();

        task_list.push(t);
        let j = serde_json::to_string_pretty(&task_list).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("tasks_list.json").unwrap();

        let result = file.write_all(j.as_bytes());
        match result {
            Ok(_) => println!("Write in success!"),
            Err(err) => println!("Problem with write in: {err:?}"),
        }
    } else {
        let mut task_list: Vec<Task> = Vec::new();
        task_list.push(t);
        let j = serde_json::to_string_pretty(&task_list).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("tasks_list.json").unwrap();

        let result = file.write_all(j.as_bytes());
        match result {
            Ok(_) => println!("Write in success!"),
            Err(err) => println!("Problem with write in: {err:?}"),
        }
    }
    Ok(())
}

fn done(task: &String) -> Result<()> {
    let mut task_json = String::new();
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open("tasks_list.json")?;
    file.read_to_string(&mut task_json)?;
    if task_json.trim().is_empty() {
        println!("to-do list is empty now!");
        return Ok(());
    } else {
        let mut task_list: Vec<Task> = serde_json::from_str(&mut task_json).unwrap();
        for t in &mut task_list {
            if t.taskname == *task {
                t.status = false;
            }
        }
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("tasks_list.json").unwrap();
        let j = serde_json::to_string_pretty(&task_list).unwrap();
        let _result = file.write_all(j.as_bytes())?;
    }
    Ok(())
}

fn output() {
    let mut file = OpenOptions::new()
        .read(true)
        .open("tasks_list.json").unwrap();
    let mut task_json = String::new();
    file.read_to_string(&mut task_json).unwrap();
    let task_list: Vec<Task> = serde_json::from_str(&task_json).unwrap();

    println!("to-do:");
    let mut cur = 1;
    for task in &task_list {
        if task.status == true {
            println!("{} {} => deadline: {}", cur, task.taskname, task.deadline);
            cur += 1;
        }
    }
    println!();
    println!("done:");
    for task in task_list {
        if task.status == false {
            println!("{} {}", cur, task.taskname);
            cur += 1;
        }
    }

}

fn main() -> Result<()> {

    let matches = Command::new("todo")
        .version("1.0")
        .about("to-do list command line application")
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(
                    Arg::new("task")
                    .help("enter the name of the task which want to add")
                    .required(true)
                ),
        )
        .subcommand(
            Command::new("done")
                .about("Mark the task which is done")
                .arg(
                    Arg::new("task")
                    .help("enter the name of the task wchich is done")
                    .required(true)
                ),
        )
        .subcommand(
            Command::new("sort")
                .about("sort and output list")
        )
        .get_matches();
    
    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let task = sub_m.get_one::<String>("task") .unwrap();
            println!("Adding task: {}", task);
            add(task).unwrap();
        },
        Some(("done", sub_m)) => {
            let task = sub_m.get_one::<String>("task").unwrap();
            println!("Marking task as done: {}", task);
            done(task).unwrap();
        },
        Some(("sort", _)) => {
            sort().unwrap();
        }
        _ => output(),
    }
    Ok(())
}

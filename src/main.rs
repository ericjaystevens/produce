use std::fs::File;                                                                                                                                                                   
use structopt::StructOpt;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(StructOpt)]
struct Cli {
    action: String,
    item: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    name: String,
}

fn main() {
    let args = Cli::from_args();
    
    let data_path = "todos.json";
    let mut todo_list:Vec<TodoItem> = load(data_path);

    match args.action.as_str() {
        "add" => new_item(args.item.trim().to_string(), &mut todo_list, data_path),
        "list" => list(&mut todo_list),
        "delete" => delete(args.item.trim().to_string(), data_path),
        _ => println!("invalid action")
    }
}

fn new_item(new_item_name: String, todo_list: &mut Vec<TodoItem>, path: &str){
    let new_item = TodoItem{
        name: new_item_name,
    };
    todo_list.push(new_item);
    save(todo_list, path);
}

//probaby should return an option or something to show the save was successful
fn save(todo_list: &mut Vec<TodoItem>, path: &str){
    let mut file = File::create(path).unwrap();
    //convert to json
    let serialized = serde_json::to_string(&todo_list).unwrap();
    //write json to file
    file.write_all(serialized.as_bytes()).unwrap();
}

fn load(path: &str) -> Vec<TodoItem> {
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let deserialized: Vec<TodoItem> = serde_json::from_str(&mut content).unwrap();
    deserialized
}

fn list(todo_list: &mut Vec<TodoItem>) {
    for todo_item in todo_list.iter().clone() {
        println!("{}",todo_item.name);
    }
}

fn delete(item_name: String, path: &str)  {
    let mut todo_list = load(path);
    todo_list.retain(|x| x.name != item_name);
    save(&mut todo_list, path);
}


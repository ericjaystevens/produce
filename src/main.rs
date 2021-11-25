use std::io;
use std::fs::File;                                                                                                                                                                   
use std::fs;    
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    action: String,
    item: String,
}


fn main() {
    
    let args = Cli::from_args();
    
    println!("first arg is {}", args.action);
    let mut todo_list = vec![];

    match args.action.as_str() {
        "add" => {
            let mut new_item = args.item.trim().to_string();
            todo_list.push(new_item);
        },
        _ => println!("invalid action")

    }

    menu(&mut todo_list);

    let mut f = File::create("output.txt").expect("Unable to create file");                                                                                                          
    for i in &todo_list{                                                                                                                                                                  
        fs::write("output.txt", i).expect("Unable to write to output file");
    }  
}

fn menu(todo_list: &mut Vec<String>){

    println!("Select Option(l - list, a - add)");    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("cannot read line"); 
    println!("input is {}",input);

    if input.trim().to_string() ==  "a" {
        println!("Enter Item to Add:");
        let mut new_item = String::new();
        io::stdin().read_line(&mut new_item).expect("cannot read line"); 
        new_item = new_item.trim().to_string();
        println!("{} added",new_item);
        todo_list.push(new_item);

        menu(todo_list);
    }
    if input.trim().to_string() ==  "l" {
        println!("list selected");
        for todo_item in todo_list {
            println!("{}",todo_item);
        }
    }
    else {
        println!("invalid option selected.");
        menu(todo_list);
    }


}

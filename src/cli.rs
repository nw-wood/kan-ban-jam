use crate::board::Board;
use regex::Regex;
use std::path::PathBuf;
use std::process;

pub fn cli_main(board: &mut Board, config_path: &PathBuf) {
	println!("board loaded; printing a quick list of items...");

    board.list_items();

    let mut _buff = String::new();

    loop { 
        _buff = String::new();
        std::io::stdin().read_line(&mut _buff).expect("couldn't read line");

        _buff.pop(); //remove new line

        let simple_inputs: Vec<&str> = _buff.split_whitespace().collect();

        let re = Regex::new(r#""([^"]+)""#).unwrap();
        let quoted_inputs: Vec<String> = re.captures_iter(&_buff)
            .filter_map(|cap| {
                let item = cap[1].to_string();
                if item.trim().is_empty() {
                    None
                } else {
                    Some(item)
                }
            })
            .collect();

        let mut inputs: Vec<String> = vec![];

        if let Some(_) = simple_inputs.get(0) { 
            inputs.push( String::from(simple_inputs[0]) );
            if quoted_inputs.get(0).is_some() {
                inputs.push(quoted_inputs[0].clone());
                if quoted_inputs.get(1).is_some() {
                    inputs.push(quoted_inputs[1].clone());
                }
            } else if simple_inputs.get(1).is_some() {
                inputs.push(simple_inputs[1].to_string());
                if simple_inputs.get(2).is_some() {
                    inputs.push(simple_inputs[2..].join(" "));
                }
            }
        }

        let inputs: Vec<&str> = inputs.iter().map(|s| s.as_str()).collect();

        //println!("quick debuggin: {inputs:?}");

        match inputs.as_slice() {
            [command, name, param] => {
                match *command {
                    "add" =>    board.add_item(name, param),
                    "rename" => board.rename_item(name, param),
                    "update" => board.update_item(name, param),
                    _ => println!("unknown command, or provided too many params"),
                }
            },
            [command, name] => {
                match *command {

                    "promote" => board.promote_item(name),
                    "demote" => board.demote_item(name),
                    "remove" => board.remove_item(name),
                    _ => println!("unknown command, or provided too many params"),
                }
            },
            [command] => {
                match *command {
                    "exit" => break,
                    "quit" => break,
                    "list" => board.list_items(),
                    "help" => println!("commands are list, help, format, rename, update, remove, promote, demote, add, and exit"),
                    "format" => println!("commands must be provided as 'command \"name wth spaces\" \"second input with spaces\"'; or simple commands as 'command name_without_spaces second input with or without spaces or quotes"),
                    // => "unknown command",
                    _ => println!("unknown command"),
                }
            },
            _ => println!("no input or all whitespace input provided; wut?"),
        }
    }

    board.save(&config_path);
    process::exit(0);
    
}
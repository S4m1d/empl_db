use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() {
    let mut deps_to_empls: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        input = input.trim_end().to_string();

        if input == "\\exit" {
            break;
        } else if input == "\\show" {
            show_data(&deps_to_empls);
        } else {
            handle_query(&input, &mut deps_to_empls);
        }
    }
}

const ADD_SYNTAX_ERR_TEXT: &str =
    "syntax error: expected Add <employee> to <department>, e.g. Add David to IT";

fn handle_query(q: &str, deps_to_empls: &mut HashMap<String, Vec<String>>) {
    let parts: Vec<String> = q.split(" ").map(|s| s.to_string()).collect();

    let mut employee: String = "".to_string();
    let mut department: String = "".to_string();

    match parts.first() {
        None => println!("empty input"),
        Some(val) => match val.as_str() {
            "Add" => match parts.get(1) {
                None => println!("{}", ADD_SYNTAX_ERR_TEXT),
                Some(val) => {
                    employee = val.to_string();
                    match parts.get(2) {
                        None => println!("{}", ADD_SYNTAX_ERR_TEXT),
                        Some(val) => match val.as_str() {
                            "to" => match parts.get(3) {
                                None => println!("{}", ADD_SYNTAX_ERR_TEXT),
                                Some(val) => department = val.to_string(),
                            },
                            _ => println!("{}", ADD_SYNTAX_ERR_TEXT),
                        },
                    }
                }
            },
            _ => println!("Unknown command: {}", val),
        },
    }

    if !employee.is_empty() && !department.is_empty() {
        deps_to_empls
            .entry(department)
            .or_insert(Vec::new())
            .push(employee);
    }
}

fn show_data(dep_to_empl: &HashMap<String, Vec<String>>) {
    println!("data:");

    for (dep, empls) in dep_to_empl {
        println!("{}", dep);

        for empl in empls {
            println!("---{}", empl);
        }
    }
}

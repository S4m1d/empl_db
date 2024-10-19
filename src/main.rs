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
        } else {
            handle_query(&input, &mut deps_to_empls);
        }
    }
}

fn handle_query(q: &str, deps_to_empls: &mut HashMap<String, Vec<String>>) {
    let parts: Vec<String> = q.split(" ").map(|s| s.to_string()).collect();

    match parts.first() {
        None => println!("empty input"),
        Some(val) => match val.as_str() {
            "Add" => execute_add_query(deps_to_empls, parts),
            "Get" => execute_get_query(deps_to_empls, parts),
            _ => println!("Unknown command: {}", val),
        },
    }
}

const ADD_SYNTAX_ERR_TEXT: &str =
    "syntax error: expected Add <employee> to <department>, e.g. Add David to IT";

fn execute_add_query(deps_to_empls: &mut HashMap<String, Vec<String>>, parts: Vec<String>) {
    let mut employee: String = "".to_string();
    let mut department: String = "".to_string();
    match parts.get(1) {
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
    }
    if !employee.is_empty() && !department.is_empty() {
        deps_to_empls
            .entry(department)
            .or_insert(Vec::new())
            .push(employee);
    }
}

const GET_SYNTAX_ERR_TEXT: &str =
    "syntax error: expected Get <employee> from <department>, e.g. Get David from IT or Get * from IT";

fn execute_get_query(deps_to_empls: &mut HashMap<String, Vec<String>>, parts: Vec<String>) {
    let mut employee: String = "".to_string();
    let mut department: String = "".to_string();

    match parts.get(1) {
        None => println!("{}", GET_SYNTAX_ERR_TEXT),
        Some(val) => {
            employee = val.to_string();
            match parts.get(2) {
                None => println!("{}", GET_SYNTAX_ERR_TEXT),
                Some(val) => match val.as_str() {
                    "from" => match parts.get(3) {
                        None => println!("{}", GET_SYNTAX_ERR_TEXT),
                        Some(val) => department = val.to_string(),
                    },
                    _ => println!("{}", GET_SYNTAX_ERR_TEXT),
                },
            }
        }
    }

    if !employee.is_empty() && !department.is_empty() {
        let result = filter_out_departments(deps_to_empls, &department);
        let result = filter_out_employees(&result, &employee);
        show_data(&result);
    }
}

fn filter_out_departments(
    dep_to_empl: &HashMap<String, Vec<String>>,
    department_pattern: &str,
) -> HashMap<String, Vec<String>> {
    let mut result_map: HashMap<String, Vec<String>> = HashMap::new();

    if department_pattern == "*" {
        return dep_to_empl.clone();
    }

    result_map = match dep_to_empl.get(department_pattern) {
        None => result_map,
        Some(employees) => {
            result_map.insert(department_pattern.to_string(), employees.to_vec());
            result_map
        }
    };

    result_map
}

fn filter_out_employees(
    dep_to_empl: &HashMap<String, Vec<String>>,
    employee_pattern: &str,
) -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();

    if employee_pattern == "*" {
        return dep_to_empl.clone();
    }

    for (department, employees) in dep_to_empl {
        for employee in employees {
            if employee == employee_pattern {
                result
                    .entry(department.clone())
                    .or_insert(Vec::new())
                    .push(employee.to_string());
            }
        }
    }

    result
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

use std::error::Error;
use std::{collections::HashMap, io};

#[derive(Debug)]
enum Operation {
    Add(Add),
    Retrieve(Retrieve),
    Help,
    Quit,
}

#[derive(Debug)]
struct Add {
    name: String,
    department: String,
}

#[derive(Debug)]
enum Retrieve {
    Department(String),
    Company,
}

impl Operation {
    fn read_command(input: &str) -> Operation {
        let mut split = input.split_whitespace();
        match split.next().unwrap().to_lowercase().as_str() {
            "add" => {
                let name = match split.next() {
                    Some(name) => name.to_string(),
                    None => return Operation::Help,
                };

                split.next();

                let department = match split.next() {
                    Some(department) => department.to_string(),
                    None => return Operation::Help,
                };

                Operation::Add(Add { name, department })
            }
            "ret" | "retrieve" => match split.next() {
                Some(department) => {
                    Operation::Retrieve(Retrieve::Department(department.to_string()))
                }
                None => Operation::Retrieve(Retrieve::Company),
            },
            "q" | "quit" => Operation::Quit,
            _ => Operation::Help,
        }
    }
}

#[derive(Debug)]
struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    fn add(&mut self, department: String, name: String) {
        let department = self.departments.entry(department).or_insert(Vec::new());
        department.push(name);
        department.sort_unstable();
    }

    fn retrieve_department(&self, department: String) -> Option<&Vec<String>> {
        self.departments.get(&department)
    }

    fn retrieve_company(&self) -> Vec<String> {
        let mut company = Vec::new();

        for department in self.departments.keys() {
            if let Some(department) = self.retrieve_department(department.to_string()) {
                for name in department {
                    company.push(name.clone());
                }
            };
        }
        company.sort_unstable();

        company
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut company = Company::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let op = Operation::read_command(&input);
        match op {
            Operation::Add(add) => company.add(add.department, add.name),
            Operation::Retrieve(Retrieve::Department(department)) => {
                match company.retrieve_department(department) {
                    Some(department) => {
                        for name in department {
                            println!("{}", name);
                        }
                    }
                    None => println!("There is no people in the department."),
                }
            }
            Operation::Retrieve(Retrieve::Company) => {
                let people = company.retrieve_company();

                if people.is_empty() {
                    println!("There is no people in the company.");
                } else {
                    for name in people {
                        println!("{}", name);
                    }
                }
            }
            Operation::Help => println!(
                "Commands:\n\
                 add <name> to <department>\n\
                 ret [department]"
            ),
            Operation::Quit => {
                println!("Bye!");
                break Ok(());
            }
        }
    }
}

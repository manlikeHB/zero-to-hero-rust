// use io::Write;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read, Write},
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Contact {
    name: String,
    phone: String,
    email: String,
}

const FILE_PATH: &str = "contact.json";

fn save_contact(contact_list: &Vec<Contact>) {
    let json = serde_json::to_string_pretty(contact_list).unwrap();
    let mut file = File::create(FILE_PATH).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn load_contact() -> Vec<Contact> {
    if let Ok(mut file) = File::open(FILE_PATH) {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn main() {
    let mut contact_list = load_contact();

    while execute(&mut contact_list) {}
    save_contact(&contact_list);
}

fn get_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn execute(contact_list: &mut Vec<Contact>) -> bool {
    println!("Choose an action: add/list/delete/search/exit");

    let input = get_input();

    manage_contact(contact_list, input)
}

fn manage_contact(contact_list: &mut Vec<Contact>, input: String) -> bool {
    let res: Vec<&str> = input.split_whitespace().collect();

    let binding = res[0].to_lowercase();
    let command = binding.as_str();

    match command {
        "add" => {
            if res.len() < 4 {
                println!("Please enter a valid contact, e.g. `add john 0234xxxx john@best.com");
                return true;
            }

            let new_contact = Contact {
                name: res[1].to_string(),
                phone: res[2].to_string(),
                email: res[3].to_string(),
            };

            contact_list.push(new_contact);

            return true;
        }
        "list" => {
            if contact_list.is_empty() {
                println!("Contact is empty!");
            }

            for i in 0..contact_list.len() {
                let contact = contact_list.get(i).unwrap();
                println!(
                    "{}. {} {} {}",
                    i + 1,
                    contact.name,
                    contact.phone,
                    contact.email
                );
            }

            return true;
        }
        "delete" => {
            if res.len() < 2 {
                println!("Usage: delete <number>");
                return true;
            }

            match res[1].parse::<usize>() {
                Ok(i) if i > 0 && i < contact_list.len() => {
                    contact_list.remove(i - 1);
                    println!("Contact Deleted!")
                }
                _ => println!("Invalid contact number"),
            };

            return true;
        }
        "search" => {
            if res.len() < 2 {
                println!("Usage: search <name>");
                return true;
            }

            let name = res[1];
            let mut found = false;
            for (i, contact) in contact_list.iter().enumerate() {
                if contact.name.contains(name) {
                    println!(
                        "{}. {} {} {}",
                        i + 1,
                        contact.name,
                        contact.phone,
                        contact.email
                    );
                    found = true;
                }
            }

            if !found {
                print!("No contact found with name containing '{}'", name);
            }

            return true;
        }
        "exit" => {
            return false;
        }
        _ => true,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_john_contact() -> Contact {
        Contact {
            name: "john".to_string(),
            phone: "090123".to_string(),
            email: "john@best.com".to_string(),
        }
    }

    fn get_mike_contact() -> Contact {
        Contact {
            name: "mike".to_string(),
            phone: "090234".to_string(),
            email: "mike@best.com".to_string(),
        }
    }

    #[test]
    fn test_manage_contact() {
        let mut contact_list = Vec::<Contact>::new();
        let john_contact: Contact = get_john_contact();
        let mike_contact: Contact = get_mike_contact();
        let input_john: String = format!(
            "add {} {} {}",
            john_contact.name, john_contact.phone, john_contact.email
        );
        let input_mike: String = format!(
            "add {} {} {}",
            mike_contact.name, mike_contact.phone, mike_contact.email
        );

        // add contact
        assert!(manage_contact(&mut contact_list, input_john));
        assert_eq!(*contact_list.get(0).unwrap(), get_john_contact());

        // list contact
        assert!(contact_list.len() == 1, "Contact list len should be 1");
        manage_contact(&mut contact_list, input_mike.clone());
        assert!(contact_list.len() == 2, "Contact list len should be 2");

        // delete contact
        assert!(manage_contact(&mut contact_list, "delete 1".to_string()));
        assert!(contact_list.len() == 1, "Contact not deleted");

        // search
        manage_contact(&mut contact_list, input_mike);
        assert!(manage_contact(&mut contact_list, "search mike".to_string()));

        // exist
        assert!(!manage_contact(&mut contact_list, "exit".to_string()));
    }
}

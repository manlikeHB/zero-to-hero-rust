use std::io;

fn main() {
    let mut list: Vec<(String, bool)> = Vec::new();
    while execute(&mut list) {}
}

fn get_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn execute(list: &mut Vec<(String, bool)>) -> bool {
    println!("Choose an action: add/list/done/remove/exit");

    let binding = get_input().to_lowercase();
    let input: Vec<&str> = binding.split_whitespace().collect();

    if input.is_empty() {
        println!("Please enter a command");
        return true;
    }

    let command = input[0];
    let opt = input[1..].join(" ");

    handle_command(list, command, opt)
}

fn handle_command(list: &mut Vec<(String, bool)>, command: &str, opt: String ) -> bool {
if command == "add" {
        if opt.is_empty() {
            println!("Task is empty!");
        }

        list.push((opt.to_string(), false));
        println!("Task added: {}", opt);
        return true;
    } else if command == "list" {
        if list.len() == 0 {
            println!("List is empty!");
            return true;
        }

        for i in 0..list.len() {
            let (task, done) = list.get(i).unwrap();
            println!(
                "{}. [{}] {}",
                i + 1,
                if *done {
                    "X".to_string()
                } else {
                    " ".to_string()
                },
                task
            );
        }
        return true;
    } else if command == "done" {
        if check_if_valid_index(list.len(), &opt) {
            match convert_to_index(&opt) {
                Some(i) => {
                    let val = list.get_mut(i).unwrap();
                    val.1 = true;
                    println!("Task {} marked as done.", opt);
                }
                _ => {
                    println!("No task found at number {}", opt);
                }
            }
        } 

        return true;
    } else if command == "remove" {
        if check_if_valid_index(list.len(), &opt) {
            match convert_to_index(&opt) {
                Some(i) => {
                    list.remove(i);
                }
                _ => {
                    println!("No task found at number {}", opt);
                }
            }
        }
        return true;
    } else if command == "exit" {
        println!("Exiting...");
        return false;
    } else {
        return true;
    }
}

fn check_if_valid_index(list_len: usize, opt: &String) -> bool {
    match convert_to_index(opt) {
        Some(val) => list_len > val,
        None => false,
    }
}

fn convert_to_index(opt: &String) -> Option<usize> {
    opt.parse::<usize>().ok().map(|x| x - 1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_index_valid() {
        let input = "3".to_string();
        assert_eq!(convert_to_index(&input), Some(2)); // 3 -> index 2
    }

    #[test]
    fn test_convert_to_index_invalid_string() {
        let input = "abc".to_string();
        assert_eq!(convert_to_index(&input), None);
    }

    #[test]
    fn test_check_if_valid_index_in_bounds() {
        let input = "2".to_string();
        assert_eq!(check_if_valid_index(3, &input), true); // list has 3, so index 1 is valid
    }

    #[test]
    fn test_check_if_valid_index_out_of_bounds() {
        let input = "5".to_string();
        assert_eq!(check_if_valid_index(3, &input), false);
    }

    #[test]
    fn test_check_if_valid_index_invalid_string() {
        let input = "not_a_number".to_string();
        assert_eq!(check_if_valid_index(3, &input), false);
    }
}

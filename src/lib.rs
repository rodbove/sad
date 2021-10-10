use std::vec::Vec;

fn substitute_content(
    content: &str,
    to_be_replaced: &str,
    to_replace: &str,
    global: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = String::new();
    for line in content.lines() {
        if line.contains(to_be_replaced) {
            if global {
                let new_line = line.replace(to_be_replaced, to_replace);
                result.push_str(&new_line);
            } else {
                let new_line = line.replacen(to_be_replaced, to_replace, 1);
                result.push_str(&new_line);
            }
        } else {
            result.push_str(&line);
        }
        result.push_str("\n");
    }

    Ok(result)
}

pub fn handle_exec_command(content: &str, exec_args: &Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let subcommand = exec_args[0];
    if subcommand != "s" {
        panic!("Invalid subcommand");
    }

    let global = exec_args[3] == "g";
    substitute_content(content, exec_args[1], exec_args[2], global)
}

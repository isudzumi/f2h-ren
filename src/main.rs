use std::env;
use std::char;

fn get_env(mut arg: env::Args) -> Result<String, String> {
    arg.nth(1).ok_or("Error".to_string())
}

fn print(result: Result<String, String>) {
    match result {
        Ok(res) => println!("{}", convert_full_width_char_to_half(&res)),
        Err(err) => println!("{}", err),
    }
}

fn convert_full_width_char_to_half(word: &str) -> String {
    let mut new_string = String::new();
    for unicode in word.chars() {
        let code_point: String = unicode.escape_unicode().collect();
        let code_num = &code_point.get(3..&code_point.len() - 1).unwrap();
        let mut num: u32 = u32::from_str_radix(&code_num, 16).unwrap();
        match num {
            0xFF00...0xFF5F => num -= 0xFEE0,
            0x3000 => num = 0x20,
            _ => {
                new_string.push(unicode);
                continue;
            }
        }
        let half_char = char::from_u32(num).unwrap();
        new_string.push(half_char);
    }
    return new_string;
}

fn main() {
    print(get_env(env::args()));
}

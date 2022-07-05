mod utils;
use utils::{_match, get_total};

fn main() {
    let result: i32 = get_seconds("2h30m20s");
    println!("{}", result);
}

fn get_seconds(_str: &str) -> i32 {
    let _string = String::from(_str);
    let mut time_array: Vec<String> = Vec::new();
    let mut string_pool = String::new();
    for _char in _string.chars() {
        string_pool.push(_char);
        if _match(_char) {
            time_array.push(string_pool);
            string_pool = String::new();
        }
    }
    get_total(time_array)
}

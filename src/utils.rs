use std::collections::HashMap;

pub fn _match(_str: char) -> bool {
    let mut flag = true;
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    if !alphabet.contains(_str) {
        flag = false;
    }
    flag
}

fn get_multiplyer(letter: char) -> i32 {
    let mut map: HashMap<char, i32> = HashMap::new();
    map.insert('d', 86400);
    map.insert('h', 3600);
    map.insert('m', 60);
    map.insert('s', 1);
    let value = map.get(&letter).unwrap();
    value.clone()
}

pub fn get_total(times: Vec<String>) -> i32 {
    let mut seconds: i32 = 0;
    for time in times {
        let multiplier: i32 = get_multiplyer(time.chars().last().unwrap());
        let number: i32 = time[0..time.len() - 1].to_string().parse::<i32>().unwrap();
        seconds += number * multiplier;
    }
    seconds
}

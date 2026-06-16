pub fn count_and_say(n: i32) -> String {
    if n == 1{
        return String::from("1")
    }
    let mut last_rle = rle(String::from("1"));
    for i in 2..n{
        last_rle = rle(last_rle);
    }
    last_rle
}

fn rle(input: String) -> String{
    let mut compressed = String::new();
    let mut previous_char = input.chars().next().expect("empty_string");
    let mut c_counter = 1;
    for current_char in input.chars().skip(1) {
        if current_char == previous_char {
            c_counter += 1;
        }
        else {
            let c_pair = format!("{c_counter}{previous_char}");
            compressed.push_str(&c_pair);
            previous_char = current_char;
            c_counter = 1;
        }
    }
    let c_pair = format!("{c_counter}{previous_char}");
    compressed.push_str(&c_pair);
    compressed
}

pub fn test1() {
    println!("placeholser");
    assert!(count_and_say(4) == String::from("1211"));
}

pub fn s_test(){
    let compressed = rle(String::from("1223334444"));
    println!("{compressed:?}");

    let check1 = count_and_say(4);
    println!("{check1:?}");
}

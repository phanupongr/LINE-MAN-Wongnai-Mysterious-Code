fn main() {
    let secret = "aWFuZ25vVzpOQU06RU5JTDp0YTpzdTpuaW9K";

    let answer = base64::decode(secret).expect("Failed to decode secret");

    let str_ans = std::str::from_utf8(&answer).expect("Invalid UTF-8");

    println!("{}", reverse_string(str_ans));
}

fn reverse_string(str: &str) -> String {
    str.chars().rev().collect::<String>()
}

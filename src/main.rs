fn main() {
    println!("Hello, world!");

    let s = String::from("HogeHoge");
    let len = string_length(&s);
    println!("{}", len);
    let _ss = s.find("og");
}

fn string_length(ss: &String) -> (usize) {
    ss.len()
}
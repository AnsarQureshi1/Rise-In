fn main() {
    let string1 = String::from("Ansar");
    let string2 = String::from("Qureshi");

    let concatenated_result = concatenate_string(&string1, &string2);
    println!("{}",concatenated_result);
}

fn concatenate_string(first:&String,second:&String) -> String {
    let mut result = String::new();
    result.push_str(first);
    result.push_str(second);
    result
}   
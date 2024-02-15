fn main() {
    let a = "This is a string literal";
    let b = String::from("This is a String");

    // convert literal to string
    let c = "converting literal to String".to_string();

    println!("{} {} {}", a, b, c);

    // concat string
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s} = {s1}-{s2}-{s3}");

    // iterating string
    let x = String::from("Hello world!");
    for i in x.chars() {
        println!("{i}")
    }
    println!("index[1] is {}", x.chars().nth(1).expect("Index not found"));
}

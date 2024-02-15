fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);

    let third: &i32 = &v[2];
    println!("v[2] -> {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("v.get(2) -> {third}"),
        None => println!("[match] There is no third element."),
    }

    println!("Vec value: {:?}", v);
}

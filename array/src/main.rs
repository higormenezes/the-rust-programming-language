fn main() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // [3, 3, 3, 3, 3]

    println!("Value of a: {:?}", a);
    println!("Value of a: {:?}", b);
    println!("Value of a: {:?}", c);
}

fn main() {
    println!("Hello, world!");
    let mut x = 25;
    let y = &mut x;
    *y = *y + 1;
    println!("{}", *y + 1);
    let z: String = String::from("Hello, World!");
    println!("{}", z);
}

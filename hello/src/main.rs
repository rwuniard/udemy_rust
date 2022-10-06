fn main() {
    println!("Hello, world!");

    let x = 5;
    {
        let x = 6;
        let y = 10;
        println!("x = {}, y = {}", x, y);
    }
    println!("x = {}", x);
}

fn main() {
    println!("Hello, world!");

    let x = 5;
    {
        let y = 10;
        println!("x = {}, y = {}", x, y);
    }
}

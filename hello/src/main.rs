use hello::greet;

fn main() {
    println!("Hello, world!");

    let x = 5;
    {
        let x = 6;
        let y = 10;
        println!("x = {}, y = {}", x, y);
    }
    println!("x = {}", x);

    // This call is explicitly using the `greet` function from the `hello` crate.
    // We can also use the `use` keyword to bring a function into scope.
    hello::greet();

    // This call is using the `use` keyword to bring the `greet` function into scope.
    greet();
}

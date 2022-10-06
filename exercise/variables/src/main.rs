const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;

    // Example to declare multiple variables.
    // For unused variables, use the underscore prefix.
    let (mut _a, mut _b): (i32, i32) = (3, 2);

    // This will generate warning since the variable is unused.
    let unused_variable = 3;

    println!("Firing {} of my {} missiles...", ready, missiles);
    // missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}

pub fn fib(a: felt252, b: felt252, n: felt252) -> felt252 {
    match n {
        0 => a,
        _ => fib(b, a + b, n - 1),
    }
}
fn main() -> felt252 {
    fib(1_felt252, 1_felt252, 1000_felt252)
}

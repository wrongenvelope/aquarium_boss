fn main() {
    println!("Hello, world!");
    let x = 5 + 5;
    println!("The variable 'x' = {}", x);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
}

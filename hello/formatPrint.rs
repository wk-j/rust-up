fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1} this is {0}", "Alice", "Bob");
    println!("{subject} {verb}", subject = "Subject", verb = "Verb");

    println!("{number:>width$}", number = 1, width=6);
}
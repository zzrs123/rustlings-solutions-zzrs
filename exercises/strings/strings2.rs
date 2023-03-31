// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a hint.

// 编译器牛逼！

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    match is_a_color_word(&word) {
        true => {
            println!("That is a color word I know!");
        }
        false => {
            println!("That is not a color word I know.");
        }
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.


// 其实还算简单，涉及的知识点是：所有权+函数调用
fn main() {
    let data = "Rust is great!".to_string();
    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}

// ChatGPT的解释
// mut data: &String中的mut表明我们可以使用引用进行修改。但是，实际上，这个引用是不可变的，因此是不能用来修改值的。这可能是代码中的错误或笔误。

// data: &mut String中没有mut，这意味着我们必须使用这个引用来修改值。这是一个可变的引用，因此我们可以修改值。

// 因此，这两种类型的引用是不同的，分别用于不同的目的。

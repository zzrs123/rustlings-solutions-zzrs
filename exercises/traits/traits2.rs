// traits2.rs
//
// Your task is to implement the trait
// `AppendBar` for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.


trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string());
        self

    }
}
/*


push()方法是将一个元素追加到Vec的末尾，
它返回值是()，而不是Vec。
在Rust中，函数的返回值类型是由最后一个表达式的返回值类型决定的，因此self.push("Bar".to_string())的返回值类型是()，与Self（即Vec<String>）不一致，编译器会报错。
因此，我们需要先将"Bar"字符串插入到Vec<String>中，再将其返回。

 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}

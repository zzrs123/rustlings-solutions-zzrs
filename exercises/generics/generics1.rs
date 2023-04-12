// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.



fn main() {
    // let mut shopping_list: Vec<&str> = Vec::new();
    let mut shopping_list: Vec<_> = Vec::new();
    // 需要使用泛型来指定vector中存储的元素类型
    // 因为shopping_list中存储的元素类型未知，
    // s所以使用占位符类型_代替泛型类型
    shopping_list.push("milk");
}

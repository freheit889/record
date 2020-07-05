fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: String) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

//或者  这里涉及到了隐式类型转换  String实现了Deref<Target=str> 所以在比较时被隐式的转换为了&str  &String与String是一致的;虽然 String没有实现copy属性 但是下面的运用没有用到word
//所以没有报错
 
fn main() {
    let word = "green"; // Try not changing this line :)
    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

//未完成  但是有一些思路
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_string() + &c.as_str().to_string(),
    }
}

mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    pub fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    pub fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }
    
    pub fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String>=words.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        println!("{:?}",capitalized_words);
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

   /* pub fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words =words.iter().map(|x| x.to_string()).collect::<Vec<String>>();// TODO
        assert_eq!(capitalized_words, "Hello World");
    }*/
}
fn main(){
    //tests::test_success();
    //tests::test_empty();
    tests::test_iterate_string_vec();
}

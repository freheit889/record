use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}
impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if(s.len()!=0){
            let s1:Vec<&str>=s.split(",").collect();
            let(name,age)=(s1[0],s1[1]);
            if(name.len()!=0 && age.len()!=0){
                return Ok(Person{name:String::from(name),age:age.parse::<usize>().unwrap()});
            }else{
                return Err("name or age error".to_string());
            }
        }else{
            return Err("all error".to_string());
        }
    }
}
fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    #[should_panic]
    fn missing_age() {
        "John,".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_age() {
        "John,twenty".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_comma_and_age() {
        "John".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name() {
        ",1".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name_and_age() {
        ",".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name_and_invalid_age() {
        ",one".parse::<Person>().unwrap();
    }

}

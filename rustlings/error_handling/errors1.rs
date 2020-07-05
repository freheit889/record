pub fn get_generate_nametag_text(name: String) -> Option<String> {
    if name.len() > 0 {
        Some(format!("Hi! My name is {}", name))
    } else {
        // Empty names aren't allowed.
        None
    }
}

fn generate_nametag_text(name: String)->String{
    match get_generate_nametag_text(name){
        Some(name)=>name,
        None=>"empty".to_string(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()),
            "Hi! My name is Beyoncé".to_string(),
        );
    }

    #[test]
    pub fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            "empty".to_string()
        );
    }
}

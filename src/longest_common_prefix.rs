
// Even performance is good this time
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::new();
    let mut index = 0;
    loop {
        let mc_char = strs[0].chars().nth(index);
        match mc_char {
            Some(c_char) => {
                for c_string in &strs[1..]{
                    let m_char = c_string.chars().nth(index);
                    match m_char {
                        Some(char) =>{
                            if char != c_char{
                                return prefix;
                            }
                        },
                        None => {
                            return prefix;
                        }          
                    }
                }
                prefix.push(c_char);
                index += 1;
            },
            None => {
                return prefix;
            }
        }
    }
}

pub fn test1(){
    let input = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];

    assert!(longest_common_prefix(input) == String::from("fl"));
}
pub fn split_string_in_half(input: String) -> (String, String) {
    let length = input.len();
    let half = length / 2;
    let first = input.chars().take(half).collect();
    let second = input.chars().skip(half).collect();
    (first, second)
}

pub fn get_common_chars(input: Vec<String>) -> char {
    let mut common_char: char = ' ';
    let first: String = input[0].clone();
    for c in first.chars() {
        let mut found = true;
        for i in 1..input.len() {
            if !input[i].contains(c) {
                found = false;
                break;
            }
        }
        if found {
            common_char = c;
            break;
        }
    }
    common_char
}

pub fn get_priority(input: char) -> i32 {
    let mut priority = 0;
    if input.is_ascii_lowercase() {
        priority = input as i32 - 96;
    } else if input.is_ascii_uppercase() {
        priority = input as i32 - 38;
    }
    priority
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string_in_half() {
        let (first, second) = split_string_in_half(String::from("abcdef"));
        assert_eq!(first, "abc");
        assert_eq!(second, "def");
    }

    #[test]
    fn test_get_common_chars() {
        let str1 = String::from("abcde");
        let str2 = String::from("axyyq");
        let str3 = String::from("eyyaq");
        assert_eq!(get_common_chars(vec![str1.clone(), str2.clone(), str3.clone()]), 'a');
        assert_eq!(get_common_chars(vec![str1.clone(), str2.clone()]), 'a');
    }
}
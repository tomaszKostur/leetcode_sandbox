

pub fn is_palindrome(x: i32) -> bool {
    // This approach seems to be disappointing in performance
    // Misusage of indexing I guess
    let x_repr = format!("{x}");
    let half_index = x_repr.chars().count() / 2;
    let mut  front = x_repr.chars();
    let mut back = x_repr.chars().rev();
    for i in 0..half_index{
        if front.next() != back.next(){
            return  false;
        }
    }
    return true;
}

pub fn test1(){
    assert!(is_palindrome(121));
    assert!(!is_palindrome(123));
    assert!(is_palindrome(3223));
    assert!(!is_palindrome(1231));
    assert!(!is_palindrome(10000021));
}
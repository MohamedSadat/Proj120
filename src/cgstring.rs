pub fn reverse_string(input: &str) -> String {
    let mut chars = input.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut j = chars.len() - 1;
    while i < j {
        let temp = chars[i];
        chars[i] = chars[j];
        chars[j] = temp;
        i += 1;
        j -= 1;
    }
    chars.iter().collect()
}

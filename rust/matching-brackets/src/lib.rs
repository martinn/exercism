pub fn brackets_are_balanced(string: &str) -> bool {
    let mut pairs: Vec<char> = Vec::new();
    for char in string.chars() {
        match char {
            '{' | '(' | '[' => pairs.push(char),
            '}' => {
                if Some('{') != pairs.pop() {
                    return false;
                }
            }
            ')' => {
                if Some('(') != pairs.pop() {
                    return false;
                }
            }
            ']' => {
                if Some('[') != pairs.pop() {
                    return false;
                }
            }
            _ => (),
        }
    }
    pairs.is_empty()
}

pub fn brackets_are_balanced(string: &str) -> bool {
    if string.is_empty() {
        return true;
    }
   let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                if let Some(last) = stack.pop() {
                    
                    match (last, c) {
                        ('(', ')') | ('[', ']') | ('{', '}') => {
                            continue;
                        }
                        _ => return false,
                       
                    }
                }
                else {
                    return false;
                }
            }
            _ => {}
        }
    }
    return stack.is_empty();
}

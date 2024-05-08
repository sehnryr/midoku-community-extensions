pub fn url_encode(s: &str) -> String {
    let mut encoded = String::new();
    for c in s.chars() {
        match c {
            'a'..='z'
            | 'A'..='Z'
            | '0'..='9'
            | '-'
            | '_'
            | '.'
            | '!'
            | '~'
            | '*'
            | '\''
            | '('
            | ')' => {
                encoded.push(c);
            }
            _ => {
                encoded.push('%');
                encoded.push_str(&format!("{:02X}", c as u8));
            }
        }
    }
    encoded
}

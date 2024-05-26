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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode() {
        assert_eq!(url_encode("aA0-_.!~*'()"), "aA0-_.!~*'()");
        assert_eq!(url_encode(" "), "%20");
        assert_eq!(url_encode("%"), "%25");
        assert_eq!(url_encode("&"), "%26");
        assert_eq!(url_encode("+"), "%2B");
        assert_eq!(url_encode("?"), "%3F");
    }
}

pub fn is_punctuation(ch: &char) -> bool {
    matches!(
        ch,
        '!' | '#'
            | '&'
            | '\''
            | '('
            | ')'
            | '*'
            | '+'
            | ','
            | '-'
            | '.'
            | '/'
            | '\\'
            | ':'
            | ';'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | '['
            | ']'
            | '^'
            | '_'
            | '`'
            | '{'
            | '}'
            | '|'
            | '~'
    )
}
// Can we remove the punctuation if it is between letters. e.g. u.s.a to usa
pub fn is_removable_punctuation(ch: char) -> bool {
    matches!(ch, '.' | '-')
}

fn punctuation_beside_whitespace(ch: &char, left: &char, right: &char) -> bool {
    is_punctuation(ch) && (left.is_whitespace() || right.is_whitespace())
}

fn beside_digits(left: &char, right: &char) -> bool {
    left.is_ascii_digit() && right.is_ascii_digit()
}

pub fn tokenize(content: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![String::new()];
    let content = content.to_lowercase();
    let mut iter = content.chars().peekable();
    let mut prev = '\0';

    while let Some(ch) = iter.next() {
        if ch.is_whitespace() {
            result.push("".to_string());
        } else {
            let next = iter.peek().unwrap_or(&' ');
            if punctuation_beside_whitespace(&ch, &prev, next) {
                result.push("".to_string());
                result.last_mut().unwrap().push(ch);
            } else if !is_removable_punctuation(ch) || beside_digits(&prev, next) {
                result.last_mut().unwrap().push(ch);
            }
        }
        prev = ch;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn whitespace_lookahead() {
        let tokens = tokenize("Hello, world!");
        assert_eq!(tokens, vec!["hello", ",", "world", "!"]);
    }

    #[test]
    fn character_lookahead() {
        let tokens = tokenize("F.R.I.E.N.D.S");
        assert_eq!(tokens, vec!["friends"]);
    }

    #[test]
    fn hyphen_deletion() {
        let tokens = tokenize("anti-discriminatory");
        assert_eq!(tokens, vec!["antidiscriminatory"]);
    }

    #[test]
    fn numbers_lookahead() {
        let tokens = tokenize("I'm in my 20's");
        assert_eq!(tokens, vec!["i'm", "in", "my", "20's"]);
    }

    #[test]
    fn decimal_percent_and_dollars() {
        let tokens = tokenize("We went up by 20.0% to 1563.53$");
        assert_eq!(
            tokens,
            vec!["we", "went", "up", "by", "20.0%", "to", "1563.53$"]
        );
    }
}

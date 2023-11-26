#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Token {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    LpStart,
    LpEnd,
    Read,
    Write,
}

impl Token {
    pub fn proc_chars(chars: &[char]) -> Vec<Token> {
        chars.iter().filter_map(Self::from_char).collect()
    }

    pub fn from_char(c: &char) -> Option<Token> {
        use Token::*;
        Some(match c {
            '>' => IncPtr,
            '<' => DecPtr,
            '+' => IncVal,
            '-' => DecVal,
            '.' => Write,
            ',' => Read,
            '[' => LpStart,
            ']' => LpEnd,
            _ => return None,
        })
    }
}

#[derive(Debug)]
pub struct TokenGroup {
    pub token: Token,
    pub ammount_merged: u32,
}

impl TokenGroup {
    pub fn new(token: Token, ammount_merged: u32) -> Self {
        Self {
            token,
            ammount_merged,
        }
    }

    pub fn group_tokens(tokens: Vec<Token>) -> Vec<TokenGroup> {
        let mut groups: Vec<TokenGroup> = Vec::new();
        let mut iter = tokens.into_iter().peekable();

        while let Some(token) = iter.next() {
            let mut amount_merged = 1;

            while let Some(&next_token) = iter.peek() {
                if token == next_token && Self::is_mergable_token(&token) {
                    amount_merged += 1;
                    iter.next();
                } else {
                    break;
                }
            }

            groups.push(TokenGroup::new(token, amount_merged));
        }

        groups
    }

    fn is_mergable_token(token: &Token) -> bool {
        use Token::*;
        matches!(token, IncPtr | DecPtr | IncVal | DecVal)
    }
}

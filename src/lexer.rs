use serde::{Deserialize, Serialize};

// non-tokens:
// - comments
// - preprocessor directives
// - macros
// - whitespace: spaces, tabs, newlines

// see: ARCHITECTURE.md Section 4 for specified lexical grammar

// note: variations are explicitly typed. Collapsing categories like keywords
//       into one variant while outsourcing variation to lexeme field on Token
//       will produce more work for syntactic analysis, since lexeme : String
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Category {
    // introductions (values)
    LiteralInt, // RE: [0-9]+
    Identifier, // RE: [a−zA−Z][a−zA−Z0−9]*

    // keywords (subset of identifiers)
    KeywordTypeInt,
    KeywordMain,
    KeywordVoid,
    KeywordReturn,

    // eliminations (operations)
    Plus,
    Minus,
    Star,
    Slash,

    // punctuation
    PuncLeftParen,
    PuncRightParen,
    PuncLeftBrace,
    PuncRightBrace,
    PuncSemiColon,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Token {
    pub lexeme: String,
    pub category: Category,
}

// TODO: keep track of file and (col, row) for error reporting
// struct Position {}

// TODO: just filter out whitespace instead of having a helper function
pub fn scan(input: Vec<char>) -> Vec<Token> {
    let cs = skip_whitespace(input);

    // literals and identifiers have arbitrary length
    // operations and punctuations are single ASCII characters
    match cs.as_slice() {
        [] => vec![],
        [f, r @ ..] => match f {
            '0'..='9' => scan_int(cs),
            'a'..='z' | 'A'..='Z' => scan_id(cs),
            '+' => {
                let t = Token {
                    lexeme: String::from("+"),
                    category: Category::Plus,
                };

                std::iter::once(t).chain(scan(r.to_vec())).collect()
            }
            '-' => {
                let t = Token {
                    lexeme: String::from("-"),
                    category: Category::Minus,
                };

                std::iter::once(t).chain(scan(r.to_vec())).collect()
            }
            '*' => {
                let t = Token {
                    lexeme: String::from("*"),
                    category: Category::Star,
                };

                std::iter::once(t).chain(scan(r.to_vec())).collect()
            }
            '/' => {
                let t = Token {
                    lexeme: String::from("/"),
                    category: Category::Slash,
                };

                std::iter::once(t).chain(scan(r.to_vec())).collect()
            }
            '(' => {
                let t = Token {
                    lexeme: String::from("("),
                    category: Category::PuncLeftParen,
                };

                std::iter::once(t).chain(scan(r.to_vec())).collect()
            }
            ')' => {
                let t = Token {
                    lexeme: String::from(")"),
                    category: Category::PuncRightParen,
                };

                std::iter::once(t).chain(scan(r.to_vec())).collect()
            }
            '{' => {
                let t = Token {
                    lexeme: String::from("{"),
                    category: Category::PuncLeftBrace,
                };

                std::iter::once(t).chain(scan(r.to_vec())).collect()
            }
            '}' => {
                let t = Token {
                    lexeme: String::from("}"),
                    category: Category::PuncRightBrace,
                };

                std::iter::once(t).chain(scan(r.to_vec())).collect()
            }
            ';' => {
                let t = Token {
                    lexeme: String::from(";"),
                    category: Category::PuncSemiColon,
                };

                std::iter::once(t).chain(scan(r.to_vec())).collect()
            }
            _ => {
                let t = Token {
                    lexeme: String::from("PANIC?"),
                    category: Category::Plus,
                };

                std::iter::once(t).chain(scan(r.to_vec())).collect()
            }
        },
    }
}

fn scan_int(input: Vec<char>) -> Vec<Token> {
    // scan_int calls skip_whitespace too to remain idempotent
    let cs: Vec<char> = skip_whitespace(input);

    match cs.as_slice() {
        [] => vec![],
        [f, _r @ ..] => match f {
            '0'..='9' => {
                #[rustfmt::skip]
                    let f = cs
                        .iter()
                        .take_while(|&&c| c.is_numeric())
                        .collect::<String>();

                #[rustfmt::skip]
                    let r = cs
                        .into_iter()
                        .skip_while(|&c| c.is_numeric())
                        .collect::<Vec<_>>();

                let t = Token {
                    lexeme: f,
                    category: Category::LiteralInt,
                };

                std::iter::once(t).chain(scan(r.to_vec())).collect()
            }
            _ => {
                // panic
                todo!()
            }
        },
    }
}

// TODO: support identifiers with alpha*numeric* characters after first alphabetic
fn scan_id(input: Vec<char>) -> Vec<Token> {
    // scan_id calls skip_whitespace too to remain idempotent
    let cs: Vec<char> = skip_whitespace(input);

    match cs.as_slice() {
        [] => vec![],
        [f, _r @ ..] => match f {
            'a'..='z' => {
                #[rustfmt::skip]
                    let f = cs
                        .iter()
                        .take_while(|&&c| c.is_alphabetic())
                        .collect::<String>();

                #[rustfmt::skip]
                    let r = cs
                        .into_iter()
                        .skip_while(|&c| c.is_alphabetic())
                        .collect::<Vec<_>>();

                let keyword = match f.as_str() {
                    "int" => Some(Token {
                        lexeme: String::from("int"),
                        category: Category::KeywordTypeInt,
                    }),
                    "main" => Some(Token {
                        lexeme: String::from("main"),
                        category: Category::KeywordMain,
                    }),
                    "return" => Some(Token {
                        lexeme: String::from("return"),
                        category: Category::KeywordReturn,
                    }),
                    _ => None,
                };

                let t = match keyword {
                    Some(k) => k,
                    None => Token {
                        lexeme: f,
                        category: Category::Identifier,
                    },
                };

                std::iter::once(t).chain(scan(r.to_vec())).collect()
            }
            _ => {
                // panic
                todo!()
            }
        },
    }
}

fn skip_whitespace(input: Vec<char>) -> Vec<char> {
    match input.as_slice() {
        [] => vec![],
        [f, r @ ..] => {
            if f.is_whitespace() {
                skip_whitespace(r.to_vec())
            } else {
                input
            }
        }
    }
}

#[cfg(test)]
fn vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    #[rustfmt::skip]
    let matching = a
        .iter()
        .zip(b.iter())
        .filter(|&(a, b)| a == b)
        .count();

    matching == a.len() && matching == b.len()
}

#[cfg(test)]
mod test_valid {
    use insta;
    use std::fs;

    use super::*;

    #[test]
    fn hello() {
        #[rustfmt::skip]
        let input = fs::read("tests/valid/hello.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect();

        let output = scan(input);
        insta::assert_yaml_snapshot!(output);
    }
}

#[cfg(test)]
mod test_invalid {}

#[cfg(test)]
mod test_skip_whitespace {
    use super::*;

    #[test]
    fn skip_space() {
        let input = "    7".chars().collect();
        let output: Vec<char> = skip_whitespace(input);
        let expected_output = "7".chars().collect();

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn skip_newline() {
        let input = r#"




        7"#
        .chars()
        .collect();
        let output = skip_whitespace(input);
        let expected_output = "7".chars().collect();

        assert!(vecs_match(&output, &expected_output))
    }
}

#[cfg(test)]
mod test_arithmetic {
    use super::*;

    #[test]
    fn simple() {
        let input = "9 + 8".chars().collect();
        let output = scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("9"), category: Category::LiteralInt },
            Token { lexeme: String::from("+"), category: Category::Plus },
            Token { lexeme: String::from("8"), category: Category::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn simple_two() {
        let input = "90 + 80".chars().collect();
        let output = scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("90"), category: Category::LiteralInt },
            Token { lexeme: String::from("+"), category: Category::Plus },
            Token { lexeme: String::from("80"), category: Category::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn complex() {
        let input = "2 + 3 * 5 - 8 / 3".chars().collect();
        let output = scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("2"), category: Category::LiteralInt },
            Token { lexeme: String::from("+"), category: Category::Plus },
            Token { lexeme: String::from("3"), category: Category::LiteralInt },
            Token { lexeme: String::from("*"), category: Category::Star },
            Token { lexeme: String::from("5"), category: Category::LiteralInt },
            Token { lexeme: String::from("-"), category: Category::Minus },
            Token { lexeme: String::from("8"), category: Category::LiteralInt },
            Token { lexeme: String::from("/"), category: Category::Slash },
            Token { lexeme: String::from("3"), category: Category::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn complex_two() {
        let input = "22 + 33 * 55 - 88 / 33".chars().collect();
        let output = scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("22"), category: Category::LiteralInt },
            Token { lexeme: String::from("+"), category: Category::Plus },
            Token { lexeme: String::from("33"), category: Category::LiteralInt },
            Token { lexeme: String::from("*"), category: Category::Star },
            Token { lexeme: String::from("55"), category: Category::LiteralInt },
            Token { lexeme: String::from("-"), category: Category::Minus },
            Token { lexeme: String::from("88"), category: Category::LiteralInt },
            Token { lexeme: String::from("/"), category: Category::Slash },
            Token { lexeme: String::from("33"), category: Category::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn complex_three() {
        let input = r#"
        23 +
        18 -
        45 * 2
        / 18
        "#
        .chars()
        .collect();
        let output = scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("23"), category: Category::LiteralInt },
            Token { lexeme: String::from("+"), category: Category::Plus },
            Token { lexeme: String::from("18"), category: Category::LiteralInt },
            Token { lexeme: String::from("-"), category: Category::Minus },
            Token { lexeme: String::from("45"), category: Category::LiteralInt },
            Token { lexeme: String::from("*"), category: Category::Star },
            Token { lexeme: String::from("2"), category: Category::LiteralInt },
            Token { lexeme: String::from("/"), category: Category::Slash },
            Token { lexeme: String::from("18"), category: Category::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }
}

#[allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Token {
	pub val: String,
	pub variant: TokenType,
	pub precedent: Option<u8>
}

impl Token {
	pub fn new(s: String, t: TokenType, prec: Option<u8>) -> Self {
		return Token {
			val: s,
			variant: t,
			precedent: prec
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
	OpenParen,
	CloseParen,
	Num,
	Op,
	Func
}

pub fn tokenize(text: &String) -> Vec::<Token>{
	let mut toks: Vec::<Token> = Vec::new();
	let mut skip: usize = 0; // to skip future iterations dynamically, im sure theres a better way to do this
	
	for (idx, c) in text.chars().enumerate() {
		if skip > 0 {
			skip -= 1;
			continue
		}
	
		match c {
			'(' => toks.push(Token::new(String::from(c), TokenType::OpenParen, None)),
			')' => toks.push(Token::new(String::from(c), TokenType::CloseParen, None)),
			'+' | '-'  => toks.push(Token::new(String::from(c), TokenType::Op, Some(2))),
			'*' | '/' => toks.push(Token::new(String::from(c), TokenType::Op, Some(3))),
			_ => {
				if c.is_ascii_digit() {
					let mut num_str: String = String::from(c);
					let mut i: usize = 1;
					while text.chars().count() > idx+i && text.chars().nth(idx+i).unwrap().is_ascii_digit() {
						num_str.push(text.chars().nth(idx+i).unwrap());
						i += 1;
					}
					skip = i - 1; // skip other digits in num so they don't become they're own tokens
					toks.push(Token::new(String::from(num_str), TokenType::Num, None));
				} else if c.is_ascii_alphabetic() {
					let mut identifier: String = String::from(c);
					let mut i: usize = 1;
					while text.chars().count() > idx+i && text.chars().nth(idx+i).unwrap().is_ascii_alphabetic() {
						identifier.push(text.chars().nth(idx+i).unwrap());
						i += 1;
					}
					skip = i - 1;
					toks.push(Token::new(String::from(identifier), TokenType::Func, None));
				} else if !c.is_ascii_whitespace() {
					panic!("Unknown identifier \"{}\" at position {}!", c, idx);
				}
			}
		}
	}
	toks
}
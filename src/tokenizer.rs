#[allow(dead_code)]

#[derive(Debug)]
pub struct Token {
	pub val: String,
	pub variant: TokenType,
	pub precedent: u8
}

impl Token {
	pub fn new(s: String, t: TokenType, prec: u8) -> Self {
		return Token {
			val: s,
			variant: t,
			precedent: prec
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
	OpenParen,
	CloseParen,
	Num,
	Op
}

pub fn tokenize(text: &String) -> Vec::<Token>{
	let mut toks: Vec::<Token> = Vec::new();

	let mut iter = text.chars().peekable();
	for c in iter {
		match c {
			'(' => toks.push(Token::new(String::from(c), TokenType::OpenParen, 1)),
			')' => toks.push(Token::new(String::from(c), TokenType::CloseParen, 1)),
			'+' | '-'  => toks.push(Token::new(String::from(c), TokenType::Op, 2)),
			'*' | '/' => toks.push(Token::new(String::from(c), TokenType::Op, 3)),
			_ => {
				if c.is_ascii_digit() {
					// TODO: Multi-digit nums
					toks.push(Token::new(String::from(c), TokenType::Num, 0));
				}
			}
		}
	}
	toks
}
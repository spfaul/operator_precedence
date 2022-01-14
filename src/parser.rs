use crate::lexer::{Token, TokenType};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
	static ref OP_TO_FUNC: HashMap<String, fn(i32, i32) -> i32> = {
		let mut m: HashMap<String, fn(i32, i32) -> i32> = HashMap::new();
		m.insert(String::from("+"), |x, y| x + y);
		m.insert(String::from("-"), |x, y| x - y);
		m.insert(String::from("*"), |x, y| x * y);
		m.insert(String::from("/"), |x, y| x / y);
		m
	};
}


pub fn parse(toks: &mut Vec::<&mut Token>) -> i32 {
	let mut res_stack: Vec::<&mut Token> = Vec::new();
	let mut t: Token;

	for tok in toks.iter_mut() {
		match tok.variant {
			TokenType::Num => res_stack.push(*tok),
			TokenType::Op => {
				let f: fn(i32, i32) -> i32 = *OP_TO_FUNC.get(&tok.val).expect("Unknown Operand!");
				if res_stack.len() >= 2 {
					let arg_b: i32 = res_stack.pop().unwrap().val.parse().unwrap();
					let arg_a: i32 = res_stack.last().unwrap().val.parse().unwrap();
					let result: i32 = f(arg_a, arg_b);
					res_stack.last_mut().unwrap().val = result.to_string();
				} else {
					panic!("Operation \"{}\" requires 2 or more arguments, not {}!", tok.val, res_stack.len());
				}
			},
			_ => {}
		}
	}
		
	return res_stack.last().unwrap().val.parse().unwrap();
}
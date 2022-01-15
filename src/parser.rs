use crate::lexer::{Token, TokenType};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::cmp;

lazy_static! {
	static ref FUNC_MAP: HashMap<String, fn(i32, i32) -> i32> = {
		let mut m: HashMap<String, fn(i32, i32) -> i32> = HashMap::new();
		m.insert(String::from("+"), |x, y| x + y);
		m.insert(String::from("-"), |x, y| x - y);
		m.insert(String::from("*"), |x, y| x * y);
		m.insert(String::from("/"), |x, y| x / y);
		m.insert(String::from("pow"), |x, y| i32::pow(x, y as u32));
		m.insert(String::from("max"), |x, y| cmp::max(x, y));
		m.insert(String::from("min"), |x, y| cmp::min(x, y));
		m.insert(String::from("mod"), |x, y| x % y);
		m
	};
}


pub fn parse(toks: &mut Vec::<&mut Token>) -> i32 {
	let mut res_stack: Vec::<&mut Token> = Vec::new();

	for tok in toks.iter_mut() {
		match tok.variant {
			TokenType::Num => res_stack.push(*tok),
			TokenType::Op | TokenType::Func => {
				let f: fn(i32, i32) -> i32 = *FUNC_MAP.get(&tok.val).expect("Unknown Operand/Function!");
				if res_stack.len() >= 2 {
					let arg_b: i32 = res_stack.pop().unwrap().val.parse().unwrap();
					let arg_a: i32 = res_stack.last().unwrap().val.parse().unwrap();
					let result: i32 = f(arg_a, arg_b);
					res_stack.last_mut().unwrap().val = result.to_string();
				} else {
					panic!("Operation/Function \"{}\" requires 2 or more arguments, not {}!", tok.val, res_stack.len());
				}
			}
			_ => {}
		}
	}

	if res_stack.len() > 1 {
		panic!("Invalid Syntax! Can only return 1 value, not {}", res_stack.len());
	}

	match res_stack.last() {
		Some(t) => t.val.parse().unwrap(),
		None => 0
	}		
}
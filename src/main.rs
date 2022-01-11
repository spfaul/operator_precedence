use std::io::Write;
mod tokenizer;

fn main() {
	let mut user_in = String::new();
	print!("Input: ");
	std::io::stdout().flush().unwrap();
	std::io::stdin().read_line(&mut user_in).unwrap();

	let toks: Vec::<tokenizer::Token> = tokenizer::tokenize(&user_in);
	// println!("{:?}", &toks);

	let rpn_toks: Vec::<&tokenizer::Token> = shunting_yard(&toks);
	println!();
	for tok in rpn_toks {
		print!("{} ", tok.val);
	}
	
}

fn shunting_yard(toks: &Vec::<tokenizer::Token>) -> Vec::<&tokenizer::Token>{
	let mut out_stack: Vec::<&tokenizer::Token> = Vec::new();
	let mut op_stack: Vec::<&tokenizer::Token> = Vec::new();

	for tok in toks.iter() {
		if tok.variant == tokenizer::TokenType::Op {
			while op_stack.len() != 0 && op_stack.last().unwrap().precedent >= tok.precedent {
				out_stack.push(op_stack.pop().unwrap());
			}
			op_stack.push(tok);
		}
		else if tok.variant == tokenizer::TokenType::Num {
			out_stack.push(tok);
		}
	}

	op_stack.reverse();
	out_stack.extend(&op_stack);

	out_stack
}

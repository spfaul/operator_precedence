use rustyline::Editor;
mod lexer;

fn main() {
	let mut rl = Editor::<()>::new();
	let user_in = rl.readline("Input: ").unwrap();

	let toks: Vec::<lexer::Token> = lexer::tokenize(&user_in);
	println!("Tokens: {:?}", toks);
	
	let rpn_toks: Vec::<&lexer::Token> = shunting_yard(&toks);
	print!("\nRPN: ");
	for tok in rpn_toks {
		print!("{} ", tok.val);
	}
}

fn shunting_yard(toks: &Vec::<lexer::Token>) -> Vec::<&lexer::Token>{
	let mut out_stack: Vec::<&lexer::Token> = Vec::new();
	let mut op_stack: Vec::<&lexer::Token> = Vec::new();

	for tok in toks.iter() {
		if tok.variant == lexer::TokenType::OpenParen {
				op_stack.push(tok);
		} else if tok.variant == lexer::TokenType::CloseParen {
			while op_stack.last().expect("Mismatched Parenthesis!").val != "(" {
				out_stack.push(op_stack.pop().unwrap());
			}
			op_stack.pop().unwrap();
		}
		else if tok.variant == lexer::TokenType::Op {
			// println!("{:?}, {:?}", tok, op_stack.last().unwrap());
			while op_stack.len() > 0 &&
				op_stack.last().unwrap().variant != lexer::TokenType::OpenParen &&
			 	op_stack.last().unwrap().precedent.expect("Tok has no Precedence!") >= tok.precedent.expect("Tok has no Precedence!") {
				out_stack.push(op_stack.pop().unwrap());
			}
			op_stack.push(tok);
		}
		else if tok.variant == lexer::TokenType::Num {
			out_stack.push(tok);
		}
	}

	op_stack.reverse();
	out_stack.extend(&op_stack);

	out_stack
}

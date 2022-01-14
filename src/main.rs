use rustyline::Editor;
mod lexer;
mod parser;

fn main() {
	let mut rl = Editor::<()>::new();
	let user_in = rl.readline("Input: ").unwrap();

	let mut toks: Vec::<lexer::Token> = lexer::tokenize(&user_in);
	println!("Tokens: {:?}", toks);
	
	let mut rpn_toks: Vec::<&mut lexer::Token> = shunting_yard(&mut toks);
	print!("\nRPN: ");
	for tok in rpn_toks.iter() {
		print!("{} ", tok.val);
	}

	println!("\nResult: {}", parser::parse(&mut rpn_toks));
}

fn shunting_yard(toks: &mut Vec::<lexer::Token>) -> Vec::<&mut lexer::Token>{
	let mut out_stack: Vec::<&mut lexer::Token> = Vec::new();
	let mut op_stack: Vec::<&mut lexer::Token> = Vec::new();

	for tok in toks.iter_mut() {
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
	out_stack.extend(op_stack);

	out_stack
}

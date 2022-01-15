# Operator Precedence Demo
A complete sample implementation of operator precedence, as well as the usage of RPN in a parser.

## Quick Start
```
cargo run
```

Send a mathematical expression in string representation to stdin (e.g. `4 - 2 * 2`).

Returns:
- Tokens Extracted From [Lexer](https://github.com/t0a5ted/operator_precedence/blob/master/src/lexer.rs)
- Input in [RPN](https://en.wikipedia.org/wiki/Reverse_Polish_notation).
- Evaluated Result

Supports:
- [x] The big 4 (`+ - / *`)
- [x] 32-bit signed integers. 
- [x] Mathematical Functions (e.g. `min`)

## Usage (Pretty much self-explanatory)
The 4 standard operators available are `+`, `-`, `*` and `/`.
Valid operator syntax is `<NUM><OPERATION><NUM>` (e.g. `4*2`, `1-3+5`)

Brackets can be used to isolate an expression, so while `4-2-2` gives `0`, `4-(2-2)` gives `4`.

The 4 standard functions available are `min`, `max`, `pow`, `mod`.
Valid function syntax is `<FUNCTION>(<NUM> <NUM>)`. All functions accept 2 32-bit signed integers as parameters and output a sigle 32-bit signed integer.

Examples:
	- `4 - 2 * 2 = 0`
	- `min(4 max(2 5)) + 1 = 5`

## References
- Shunting Yard Algorithm: https://en.wikipedia.org/wiki/Shunting-yard_algorithm
- RPN: https://en.wikipedia.org/wiki/Reverse_Polish_notation

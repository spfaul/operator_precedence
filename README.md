# operator-precedence playground
For the sake of learning about the innerworkings of compilers.

## Quick Start
```
cargo run
```

Send a mathematical computation in string representation to stdin (e.g. `4 - 2 * 2`).
Returns input in [RPN](https://en.wikipedia.org/wiki/Reverse_Polish_notation).

Supports:
	[x] The big 4 (`+ - / *`)
	[x] Multi-digit Numbers
	[ ] Mathematical Functions (e.g. `min`, `max`)

## References
- Shunting Yard Algorithm: https://en.wikipedia.org/wiki/Shunting-yard_algorithm
- RPN: https://en.wikipedia.org/wiki/Reverse_Polish_notation

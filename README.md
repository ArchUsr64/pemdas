# pemdas
Pemdas evaluator written in rust
### Supported Operations
| Operation | Symbol |
|     -     |  :-:    |
| Addition  |   +    |
| Subtraction  |   -    |
| Multiplications  |   *    |
| Division  |   /    |
| Exponentiation  |   ^    |

**Note:** Unary operations with `+` and `-` symbols are not yet supported

## Building and Execution
1. Install [rust](https://rust-lang.org) toolchain for your system  
2. Clone the repository  
   `git clone https://github.com/ArchUsr64/pemdas`  
3. Execute using `cargo`  
   `cargo run --bin calculator`
4. Enter an expression into the Standard Input   
   `5*(9+3)`

### Todo:
- [x] Binary Operators
- [x] AST Creation
- [x] AST Evaluation
- [ ] Unary Operators
- [ ] Publish to [crates.io](https://crates.io)
  - [x] Error Handling
  - [ ] Documentation
- [ ] Pre-processing inputs for `calculator`

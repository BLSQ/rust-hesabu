# rust-hesabu

try to port to rust [go-hesabu](https://github.com/BLSQ/go-hesabu)
for fun and perhaps profit (let's hope better perf)

# development
using [evalexpr](https://github.com/ISibboI/evalexpr)
try port this algo https://github.com/ISibboI/evalexpr/issues/37

## 

## TODO
- [input/output as json](https://rust-lang-nursery.github.io/cli-wg/in-depth/machine-communication.html#json-output-for-machines
)
- error handling (when parsing, evaluating,...)
- array support ?
- other functions
  - if
  - avg,...

## current dev notes

```
cargo build
time ./target/debug/rust-hesabu 
PROCESSING : c => a + 10 * b
a
b
PROCESSING : a => 10
PROCESSING : b => 10+a
a
PROCESSING : d => a + sin(b)
a
sin
b

real	0m0.003s
user	0m0.000s
sys	0m0.000s
```

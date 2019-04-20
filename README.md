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
cargo build --release && time cat test.json | target/release/rust-hesabu
{"d":{"Float":31.0},"a":{"Float":10.5},"b":{"Float":20.5},"e":{"Int":10},"c":{"Float":215.5}}
```

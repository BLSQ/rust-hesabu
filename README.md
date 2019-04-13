# rust-hesabu

try to port to rust [go-hesabu](https://github.com/BLSQ/go-hesabu)
for fun and perhaps profit (let's hope better perf)

# development
using [evalexpr](https://github.com/ISibboI/evalexpr)
try to https://github.com/ISibboI/evalexpr/issues/37

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
# rust-hesabu

try to port to rust [go-hesabu](https://github.com/BLSQ/go-hesabu)
for fun and perhaps profit (let's hope better perf)

# development
using [evalexpr](https://github.com/ISibboI/evalexpr)
try port this algo https://github.com/ISibboI/evalexpr/issues/37

## 

## Solver algorithm

 the algorithm to solve the problems is as follow:

Given the following problem

```json
{
  "c": "a + 10 * b",
  "a": "10",
  "b": "10+a"
}
```

**1.** parse all equations and extract their dependencies

```json
{
  "c": ["a","b"],
  "a": [ ],
  "b": [ "a" ]
}
```

**2.** to deduce the order of evaluation of the various expressions, we do a reverse [topological sort](https://github.com/otaviokr/topological-sort#topological-sort) based on these dependencies
```json
["a", "b", "c"]
```
example implementation in [rust](https://github.com/gifnksm/topological-sort-rs/blob/master/src/lib.rs#L305)

**3.** evaluate the equations one by one and store their values in the context for further reuse

*3.1* evaluate "10" and store as "a"
```
{
  "a": "10"
}
```
*3.2* evaluate "10+a" and store as "b"
```
{
  "a": "10"
  "b": "20"
}
```
*3.3* evaluate  "a + 10 * b" and store as "c"
```
  "a": "10",
  "b": "20",
  "c": "210"
```
**4.** solution complete !

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

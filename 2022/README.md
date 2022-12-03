# advent of code 2022 in rust

## benchmarks

see [BENCHMARKS.md](BENCHMARKS.md)

## running 

### all 

`cargo run`

### specific day

`cargo run <day number as 2 digits>`

## benchmarking

`cargo bench`

## regenerating BENCHMARKS.md

nushell: `cargo criterion --message-format=json | criterion-table | save BENCHMARKS.md`

bash: `cargo criterion --message-format=json | criterion-table > BENCHMARKS.md`
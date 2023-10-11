# Regox

A dubiously-named highly-experimental barely-functional bridge between Ruby and a very small part of the Rust [regex] crate.

## Misleading Figures

Extracting image data from a 1.8MB Google results page:

```
Warming up --------------------------------------
        Regex match?     1.042k i/100ms
        Regox match?     6.847k i/100ms
Calculating -------------------------------------
        Regex match?     10.427k (± 0.7%) i/s -     53.142k in   5.096958s
        Regox match?     68.707k (± 4.1%) i/s -    349.197k in   5.093921s

Comparison:
        Regox match?:    68707.4 i/s
        Regex match?:    10426.7 i/s - 6.59x  slower

Warming up --------------------------------------
          Regex scan    52.000  i/100ms
          Regox scan    74.000  i/100ms
Calculating -------------------------------------
          Regex scan    531.275  (± 4.1%) i/s -      2.652k in   5.000920s
          Regox scan    741.995  (± 0.4%) i/s -      3.774k in   5.086372s

Comparison:
          Regox scan:      742.0 i/s
          Regex scan:      531.3 i/s - 1.40x  slower
```

Matching a sample ReDoS pattern against a 2k text:

```
Warming up --------------------------------------
        Regex match?   335.000  i/100ms
        Regox match?    18.500k i/100ms
Calculating -------------------------------------
        Regex match?      2.340k (± 0.2%) i/s -     11.725k in   5.010792s
        Regox match?    184.936k (± 0.1%) i/s -    925.000k in   5.001731s

Comparison:
        Regox match?:   184936.0 i/s
        Regex match?:     2340.0 i/s - 79.03x  slower

Warming up --------------------------------------
          Regex scan   234.000  i/100ms
          Regox scan    16.503k i/100ms
Calculating -------------------------------------
          Regex scan      3.008k (±12.9%) i/s -     14.742k in   5.005266s
          Regox scan    166.052k (± 0.3%) i/s -    841.653k in   5.068652s

Comparison:
          Regox scan:   166052.2 i/s
          Regex scan:     3008.3 i/s - 55.20x  slower
```

[regex]: https://github.com/rust-lang/regex
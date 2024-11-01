# yuri

![meme why uri when you can have yuri](https://cdn.jsdelivr.net/gh/yaws-rs/ytypes@main/yuri/assets/yuri-logo.jpg)

no_std, alloc-free permissive URI Parser & Builder

```rust
let s = "https://foo:secret@foobar.test:666/?q=a&m=s#fragemnt";
let yuri::uri = Uri::new(s).expect("Failed to parse URI");
```

## Motivation

Optimise on size and being permissive and flexible minimally parsing in no_std & alloc-free environments.

Various RFCs / standards & intepretations complicate the picture,

e.g. punycode and this is left to the downstream consumer currently to validate.

We may in the future support opt-in further validation (e.g. IDNA), which the downstream consumer must consider.

## Benchmark

In MacBook M1 13":

| Scenario                     | Criterion                       |
| :---                         | :---                            |
| yuri::Uri New full HTTPs URL | [65.657 ns 65.751 ns 65.846 ns] |

## RFCs

| RFC       | Status | Description |
| :---      | :---   | :---        |
| [rfc3986] | must   | 2005 / Uniform Resource Identifier (URI): Generic Syntax |
| [rfc6570] | maybe  | 2012 / URI Template / variable expansion                 |
| [rfc8820] | ?      | 2020 / URI Design and Ownership                          |
| [rfc8615] | ?      | 2019 / Well-Known Uniform Resource Identifiers (HTTP)    |

[rfc3986]: https://www.rfc-editor.org/rfc/rfc3986.html
[rfc6570]: https://www.rfc-editor.org/rfc/rfc6570.html
[rfc8820]: https://www.rfc-editor.org/rfc/rfc8820.html
[rfc8615]: https://www.rfc-editor.org/rfc/rfc8615.html

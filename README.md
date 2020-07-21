# DevCalc

Developer calculator with CLI and step by step display of the operations.

## Install

To install this package please make use of Cargo:

```bash
cargo install --git https://github.com/edoren/DevCalc --tag v0.1.0
```

## Operators

| Operation          | Operator |
|--------------------|----------|
| ADDITION           | `+`      |
| SUBTRACTION        | `-`      |
| MULTIPLICATION     | `*`      |
| DIVISION           | `/`      |
| BINARY AND         | `&`      |
| BINARY XOR         | `^`      |
| BINARY OR          | `\|`     |
| BINARY SHIFT LEFT  | `<<`     |
| BINARY SHIFT RIGHT | `>>`     |

## Number Prefix

In order to execute expressions the program detect the number in different bases 

| Base        | Prefix | Example       |
|-------------|--------|---------------|
| Binary      | `0b`   | **0b**1101110 |
| Octal       | `0o`   | **0o**1647023 |
| Decimal     | None   | 123456789     |
| Hexadecimal | `0x`   | **0x**123CDEF |

Note that hexadecimal supports both upper and lower case letters

## Examples

```bash
devcalc "(0xFF - 1) << 4"
devcalc "0xEE"
devcalc "0b10 + (2 * 0o3)"
```

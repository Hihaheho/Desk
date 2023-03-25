# Desk Programming Language Formal Definition

!Note: not an easy-understanding guidance

## Syntax

```bnf
tn :== (number type)
| Nat | Int | Rat | Real
| uN | iN (n-bit integer)
tt :==
| U (universe)
| { t1, ... } (unordered set of types)
f : `tt` or super set of t with any t in it is replaced to `t | tn` (type family)
t :==
| tn
| tn < t
| tn < t
| ' Char '
| Π f
| Σ f
| λ t → t
| Ident (variable)
| [ t1, ... ] (fixed-length vector)
| [ t; t_len ] (free-length vector)
| < t_key1 => t_value1, ... > (map)
| let t_def in t
| & t t1, ... (application)
| branch t { t_case1 => t1, ... }
| ! t ~> t_out (perform)
| handle t { t_in1 ~> t_out1 => t1, ... }
| @ Ident t (label)
| @@ Ident t (brand)
| t_from is t_to (exploit type coercion)
| ? (hole)
| # Dson t (attribute)
```

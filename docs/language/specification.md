# Desk Programming Language Specification

This is an official formal specification of Desk programming language (aka. Desk-lang).
It's intended for implementation or academic use and not for easy-understanding guidance.

## Tokens

Whitespaces separate tokens.

```ebnf
whitespaces = ? any UTF-8 whitespaces ?;

comment-token = "(", { ( comment | comment-charactor ) }, ")";
comment-charactor = ? printable UTF-8 charactors except "(" and ")" and whitespaces ?;

ident-token = ident-unit, { whitespaces, ident-unit };
ident-start = ? a printable UTF-8 charactor except ASCII symbols and numbers" ?;
number = ? 0-9 ?;
ident-charactor = ident-start | number | "-" | "_" | "'" | "@";
ident-unit = ident-start, { ident-charactor };

int-token = [ "-" ], non-zero-number, { number };
non-zero-number = ? 1-9 ?;

string-token = '"', escaped, '"'
escaped = ? escaped characters like \"aa\n\" ?;

float-token = ? TBA ?;
uuid-token = ? UUID ?;
divide-token = "/";
let-token = "$";
in-token = "~";
perform-token = "!";
this-token = "'this";
from-here-token = "^";
type-annotation-token = ":";
trait-token = "%";
attribute-token = "#";
sum-token = "+";
product-token = "*";
minus-token = "-";
comma-token = ",";
dot-token = ".";
apply-token = ">";
reference-token = "&";
array-begin-token = "[";
array-end-token = "]";
set-begin-token = "{";
set-end-token = "}";
hole-token = "?";
infer-token = "_";
handle-token = "'handle";
continue-token = "<!";
lambda-token = ? "\" ?;
arrow-token = "->";
effect-arrow-token = "=>";
import-token = "'import";
export-token = "'export";
brands-token = "'brands";
type-token = "'type";
number-type-token = "'number";
string-type-token = "'string";
brand-token = "@";
alias-token = "'alias";
a-token = "'a";
card-token = "'card";
```

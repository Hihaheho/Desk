# Introduction of Desk-lang

Note: Desk-lang specifies only its syntax and semantics, so the example codes includes pseudo built-in functions/effects with recommended signatures.

Desk-lang has the following characteristics:
- strongly dependently typed: term is type and type is term
- true pure functional language with algebraic effects: any effect are tracked by type system and handleable.
- structural over nominal: things has no name, so you need to describe it completely to distinguish it with others
- simple over easy: impossible to do the same thing in another way
- the most minimalist general programming language: learning just one word in a day for a month, Desk-lang would be your first language you know the every page of a dictionary.
- no surprising: the code itself and type conclusions describes everything about your program
- unphysical programming: you can forget about the physical world including any computers while enjoying Desk-lang.

## Literals

Desk-lang has the following literals:
``` 
42 // nat
-1 // int
1/3 // rat
π // real
[1, 2, 3] // a list of nat
<a => 10, f => 15> // a character to nat map
{ 1, 2, 3 } // a set of nat
```

## Aliases

Here is the incomplete function definition that receives an natural number and just returns it (you don't need to understand it for now).
```
Πid &id nat
```

It's incomplete because Desk-lang doesn't have `nat` syntax because things have no name. To complete the definition, we need a substitution rule to make `nat` as an alias of the full definition of natural numbers.

```
nat = Σ { 0, 1, ... }
```
With this rule, the definition of the function is complete, and, without this substitution rule, the expression makes no sense at all.

The above is just a basic usage of alias. 他にも様々な使い方があり強力な機能です。この記事でも色々と活躍しますが、実際にDesk-langを書く段になると、必要不可欠なものに感じるでしょう。
しかし単なる置き換えであることには変わらないので型付や評価をする前に全て除去され、その除去された状態を最初から書いておくこともできるので本質的に必要なものではありません。

また、エイリアスは再帰的に置き換えられますが、循環依存は禁止されています。またエイリアスにより新しいもの構文の導入も可能ですが、パターンマッチして取り出したものを別の表現に移すだけで、Desk言語における計算を行うことはできません。

## Terms, Types, Types of Types

In Desk-lang so-called terms and types share the syntax, and types of types, type of type of type, ... also do, and they would be mixed in an expression in any way.

That infinitive `type of` can be defined like:
```
type := term | TypeOf(type) 
```

Yes, this equals to Peano numbers, and the number can be used to denote the rank of type.

In Desk-lang, "type" of that definition is used anywhere.

## Π and Σ

It's hard to describe or understand this syntax without examples.

### `Πid { nat => &id nat }`
`id` is the context and `{ nat => &id nat }` is the family.
It's the same as `fn (x: Nat) -> x` in functional languages, `λ x: nat. x` in lambda calculus, and `Union of all x where x in N`.
`id` in `&id nat` must be matched with one in `Πid`, and `&id nat` means "reference nat in the `id` context, and extract an element of nat.

The notation is a little bit redundant in such a case, and we can denote it as `Π& nat` with alias `Π& x = Πa &a x (where x is any expression and a is a fresh context name)`. Also we can create one for `Π&` in the same way.

### `char = Σ& {a, b, あ, 🪑, ...}`

It creates an alias for character type. 

### `str = Σs [ &s @1 char, &s @2 char, * ]`

It creates an alias for string type as arbitrarily length list of characters.

### `Π{ t → t1, * } = Π& { t → t1, * }`

Record syntax

直積リテラル構文の定義

### `λ x → y =

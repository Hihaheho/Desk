# Desk Programming Language

Note: 🚧 means "under development".

## Resources

- [Specification](/docs/language/specification.md)

## Features

Desk-lang is a statically typed programming language with;

- minimal syntax and semantics (like Lisp)
- algebraic data types
- [bidirectional typechecking](https://arxiv.org/abs/1306.6032)
- structural subtyping (like TypeScript)
- algebraic effects and handlers
- 🚧 structural trait
- content-addressed code (like [Unison](https://www.unison-lang.org/learn/the-big-idea/))
- 🚧 metaprogramming support

## Basic syntax

a comment:
```desk
(this is a comment)
```

a type annotated expression:
```desk
^ expr : type
```

## Basic types

`'integer`
```desk
42
```

`'string`
```desk
"hello world"
```

`['integer']` an array of numbers
```desk
[1, 2, 2]
```

`{'integer}` a set of numbers
```desk
{1, 2}
```

`* 'string, 'integer` a product type
```desk
* "Ryo", 24
```

`[+ 'integer, 'string]` a sum type
```desk
[1, "a"]
```

`@name 'string` a labelled type
```desk
@name "Ryo"
```

## Function

a function:
```desk
\ x -> expr
```

calling a function:
```desk
> type arg1, arg2
```

an identity function:
```desk
$ \ x -> x: id ~
> id "a"
```

## Let in

let syntax:
```desk
$ 42 ~ (let 42 in)
& 'integer (this is 42)
```

let with a type variable:
```desk
$ "Ryo": a name of me ~
& a name of me (this is "Ryo")
```

## Match

```desk
+ &x
'integer ->
  "this is number"
'string ->
  "this is string"
```

If branches have different output types, they will be summed.
The type of this expression is `+ 'string, 'string` simplified to `'string`.

## Effects

Performs an effect
```
! expr
```

The type of the below expression is `b ! {`
```
! ^expr: in => out
```

## Handlers

## Trait

## Resources

- [Specification](/docs/language/specification.md)

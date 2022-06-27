# Desk Programming Language

Note: 🚧 means "under development".

## Features

Desk-lang is a statically typed programming language with;

- minimal syntax and semantics (like Lisp)
- algebraic data types
- [bidirectional typechecking](https://arxiv.org/abs/1306.6032)
- structural subtyping (like TypeScript)
- algebraic effects and handlers
- 🚧 structural trait
- content-addressed code (like [Unison](https://www.unison-lang.org/learn/the-big-idea/))

## Basic types

Note: `^expr: type` is the type annotation syntax.

```desk
^42: 'number
^"hello world": 'string
^[1, 2, 2]: ['number] (an array)
^{1, 2}: {'number} (a set)
^* "Ryo", 24: * 'string, 'number (a tuple of a string and number)
^[1, "a"]: [+ 'number, 'string] (an array of strings and numbers)
^@name "Ryo": @name 'string (a labelled type)
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

## Let in

let syntax:
```desk
$ 42 ~ (let 42 in)
& 'number (this is 42)
```

let with a type variable:
```desk
$ "Ryo": a name of me ~
& a name of me (this is "Ryo")
```

## Match

```desk
+ &x
'number ->
  "this is number"
'string ->
	"this is string"
```

If branches have different output types, they will be summed.

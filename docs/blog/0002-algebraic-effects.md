# Algebraic Effects and Handlers on Desk

Desk-lang is a language with algebraic effects and handlers.

This article will go into depth about the `effects` of the Desk-lang.

The title's "on Desk" means both:
- "in Desk-lang"
- "on your desk" (easy tutorial)

## TL;DR

In general:

- Effects can represent exceptions, monads, async/await, coroutines.
- An effect has an input and an output.
- A handler handles an effect.
- An effect is like an interface, so it's handler-agnostic.
- An effect has a continuation, so a handler can resume it.

In Desk-lang:

- It's something like a true pure functional language because every side effect is always tracked by the type system.
- Its strong type-inference system covers effects.
- Effects are distinguished by input/output type.
- `perform` and `continue` are symmetric ("resuming a continuation" is also "performing an effect").
- A continuation is so-called multi-shot continuation (useful for non-deterministic computations, etc.).
- Effects can be treated like a system call and used to communicate with external world.
- Higher-order functions can be typed with care for effects.

## What are algebraic effects and handlers?

Please read [this paper](https://www.eff-lang.org/handlers-tutorial.pdf). (Excuse me, I'm just kidding.)

To put it bluntly, effects are like, you know, React Hooks![^criticized]

With just a single, simple language feature called `effect`, you can express things like exceptions, monads, async/await, coroutines, non-deterministic calculations, and many other things. So the language will be simple.

There is another simple explanatory article[^overreacted], but our post is more aimed at a simple tutorial with **real codes** and technical details.

A language can be a purely functional language like Desk-lang by implementing the following features:
- Every side effect is treated as an `effect`.
- Every `effect` is tracked by the type system. 

Also, Desk-lang's type system has a strong inference system that covers effects.
This allows us to know what effect will occur just by looking at the inferred type.

## Effect

This is the representation of an effect in Desk-lang (where $\tau$, $\tau_1$, etc. will represent any type).

$$\tau_1 \ \leadsto \ \tau_2$$

The left side of the arrow ( $\tau_1$ above) is called the "input" of the effect, and the right side ( $\tau_2$ above) is called the output of the effect.

In other words, in Desk-lang, every side effect has an input and an output.

Also, in Desk-lang, the distinction between side effects is made by the type of input and output.

Causing an effect is called `perform`. As shown below, `! expr ~> type` is a `perform` syntax.
```desk
! 1 ~> 'string
```
This expression performs an effect with ``1`` as the input and waits for the output of `'string` type.
By the way, this effect is; $$\texttt{'integer} \ \leadsto \ \texttt{'string}$$ and the expression is typed as `'string` (but with effects).

## Examples of effects

Let's look at various effect examples as it's faster to understand what are effects.

### print effect (to print a string to somewhere)

This is `print` effect. The input is `'string` and the output is `@"printed" *<>`.

$$
\texttt{'string} \ \leadsto \ \texttt{@"printed"} \ \texttt{*<>}
$$

`@"printed"` is a label attached to the type, and `*<>` is an empty tuple (Rust's `()`). If you write it in a Rust-like style, it will look like this:

```rust
&str ~> Printed<()>
```

It takes a string and returns `()`, so it's similar to `println!`. It actually behaves the same way.

**Note**
> `print` effect can be used to print **anywhere**.
> This is because you can specify handlers such as `print-to-stdout handler` and `print-to-screen handler` (`handler` is described later).
> On the other hand, if you want to distinguish the destination in the code, just rewrite `@"printed"` to `@"printed to stdout"` or `@"printed to screen"`.

### get/set (to get/set a state)

`get` effect is as follows:

$$
\texttt{\*<>} \ \leadsto \ \texttt{+<} \ \texttt{@"state"} \ \tau \ , \texttt{@"none"} \ \texttt{\*<>} \texttt{>}
$$

`set` effect is as follows:

$$
\texttt{@"state"} \ \tau \ \leadsto \ \texttt{\*<>}
$$

Rust-like rewrites of each are as follows:

```rust
get: () ~> enum { State(T), None }
```

```rust
set: State<T> ~> ()
```

`+<>` is equivalent to `enum`. You may be surprised that `enum` suddenly appeared, but in Desk-lang, there is no concept of pre-defining types, so it becomes like the above when rewritten in Rust.

I think that both `get` and `set` are intuitive in terms of input and output. If you can feel that Desk-lang's `effect` seems to be intuitive, it will be a great success.


**Note**
> These `get`/`set` are storage-agnostic as `print` is destination-agnostic.
> If you want to distinguish storage in the code, just rewrite `@"state"` to `@"app global state"` or `@"thread local state"`.

### throw (to throw an exception)

Here is `throw` effect:

$$
\texttt{@"exception"} \ \tau \ \leadsto \ \texttt{\*<>}
$$

I guess you can read this without further explanation.
An exception object is an input and `*<>` is an output.
It seems somewhat intuitive, but I think some people feel uncomfortable.
The cause of the discomfort is that there is an "output".
Generally, exceptions are thrown and delegated exception handling to the caller. In that case, there is nothing that looks like "output".

Desk-lang's effect always has an output, and Desk-lang uses effects to represent exceptions, and the input is an exception object.
Then what does the "output" represent?
I will answer this question later, so for now, ignore it.

Also, we haven't talked about exception handling yet. `throw` and `catch` should be a pair, and they are a pair also in Desk-lang, `throw` is an `effect` and `catch` is a `handler`. Now, let's talk about the `effect`'s counterpart, `handler`.

## Handler

A handler is something that handles a specific effect. The following is (compilable) code in Desk-lang to do so.

```desk
'handle ^`effectful function`(*<>) {
  'string ~> @"printed" *<> => <~! @"printed" *<> ~> *<>,
  @"division by zero" 'integer ~> 'integer => "ok",
}
```

Rust-like rewrite is as follows:

```rust
handle effectful_function(()) {
  &str ~> Printed<()> => continue Printed<()> ~> (),
  division by zero<i32> ~> i32 => "ok",
}
```

I used a lot of syntax that I haven't explained yet, so let's explain them all at once.

``^`effectful function`(*<>)`` calls a function of type `` `effectful function` `` with `*<>` as a parameter.

**Note**
> In Desk-lang, you can't create a function without arguments, so in such cases you need to create a function that explicitly receives `*<>` as in this example.

`'handle expr { effect => handler, ... }` is a `handle` expression.

In this example, we handle two effects, `'string ~> @"printed" *<>` and `@"division by zero" 'integer ~> 'integer`.
The former is `print` effect described above, and the latter is, you may guess, a division by zero exception.
Although it is an exception, there is an output here, and it's not `*<>` (described later).

`<~! expr ~> type` is `continue` syntax. In fact, you can write the same thing with `perform` syntax, but it is more convenient to use this one. Later, I will explain how to use `perform` operator instead of `continue`.

That's the end of the syntax explanation. Let's think about how this program actually works.

First, the function called by the `handle` expression seems to have two side effects. Probably the type of the function is as follows.

$$
\backslash \ \texttt{\*<>} \ \rightarrow \ \texttt{!} \ \\{
\ \texttt{'string} \ \leadsto \ \texttt{@"printed"} \ \texttt{\*<>},
\texttt{@"division by zero"} \ \texttt{'integer} \ \leadsto \ \texttt{'integer}
\ \\} \ \texttt{\*<>}
$$

Oops, I surprised you. First, let's look at the type ignoring the effect.

$$
\backslash \ \texttt{\*<>} \ \rightarrow \ \texttt{\*<>}
$$

It's very easy to understand. In Rust, it's like this:

```rust
fn(()) -> ()
```

If we look back at the original one, we can see that the difference is the part `! { ... }`.
This is a set of effects that the expression may perform, and this is inferred by the type system.
As the type is like this, we can imagine that there are expressions such as `! "hello world" ~> @"printed" *<>` and `! @"division by zero" 1 ~> 'integer` inside the function.

First, what happens when `! "hello world" ~> @printed *<>` is called inside the function?

The correct answer is "nothing happens, and the function continues its processing as it is."
The function side performed `print` effect, but the handler did not output anything
and passed the output `@"printed" *<>`, and the target expression resume the processing.

I explained that effects are expressed as $\tau_1 \ \leadsto \ \tau_2$.
This is like an interface, and as long as the interface is followed, the handler can do anything.
This is the characteristic of algebraic effects and handlers.

Next, what happens if `! @"division by zero" 1 ~> 'integer` happens inside the function?

The correct answer is "the execution of the function is halted, and the evaluation result of the whole `handle` expression becomes `"ok"`.

**Note**
> Of course, the type of whole `'handle` expression is inferred to be `+<'string, *<>>`.

## Why is "output" required for exceptions?

By the above, I think you've guessed why the output is required for exceptions in Desk-lang.
In short, it's because Desk-lang's effect has a continuation.
We can `perform` an effect with "input", and we can continue the processing by receiving "output" from the handler.

In other words, exceptions also have a continuation like this:

```desk
'handle ^`function may have zero division`(*<>) {
  @"division by zero" 'integer ~> 'integer => <~! 0 ~> *<>,
}
```

If you run this, even if division by zero occurs, the expression will not fail and be halted, and the result of the calculation such as `3 / 0` will be `0`, the output passed by the handler.

## Symmetric perform and continue

As I explained earlier, we can use `perform` expression instead of `continue` expression. Here is the example:

```desk
'handle ^`effectful function`(*<>) {
  'string ~> @"printed" *<> => ! @"printed" *<> ~> *<>,
}
```

This also compiles and works as expected.
The difference from before is that we changed `<~!` to `!`.

Yes, `perform` and `continue` are symmetric.
In other words, "continue the processing by passing an output value" equals "performing an effect with the value".

Since we only changed `<~!` to `!`, the number of keystrokes has decreased.
However, in general, use `<~!`.
Not only `<~!` operator can make it clear to the reader that you are continuing,
but also the compiler checks that you are correctly continuing the effect.

On the other hand, when do you have to use `!` to continue? In fact, `<~!` can only be used in the `handle` expression.
Therefore, when you want to define a handler external, you need to use `!`.

**Note**
> By using polymorphism like the following, you can reuse the same handler for any expression.
>
> ```desk
> 'forall output \ *<> -> ! { @"division by zero" 'integer ~> output } *<>
> ```

## Multi-shot continuation

As I explained earlier, we can pass output to effect and continue the processing.
In fact, we can pass output multiple times like:

```desk
'type add \ *<@1 'integer, @2 'integer> -> @"sum" 'integer
'handle (
  $ ! "a" ~> 'integer;
  :'integer ^add(&'integer, &'integer)
) {
  'string ~> 'integer =>
    ^add(<~! 1 ~> 'integer, <~! 2 ~> 'integer)
}
```

It's a little complicated. Let's rewrite it in Rust-like style:

```rust
handle {
  let number = ! "a" ~> i32;
  number * 2
} {
  'string ~> 'integer => {
    <~! 1 ~> i32 + <~! 2 ~> i32
  }
}
```

This code performs an effect with `"a"` as input, and doubles the output.
In the handler, we continue twice and add the two results.

Now, what is the result of this expression?

Let's answer the question by using code transformation that does not change the result of the expression.

In this snippet:

```rust
<~! 1 ~> 'integer + <~! 2 ~> 'integer
```

We can transform it to:

```rust
(1 * 2) + (2 * 2)
```

So, the result of the expression is `6`.

**Note**
> `:type expr`, which the example uses, is a type annotation syntax and like `(type) expr` in C.
> The code makes `@"sum" 'number` to `'number`.
> 
> Of course, the compiler denies invalid type annotations like `:'number "a"`.

## Effect as a system call

In fact, you cannot write a program with real side effects in Desk-lang alone.
Although you can use effects in Desk-lang, you cannot read and write files, get the current time, or spawn a lightweight process.

However, there is one way to add such side effects to Desk programs.
That is to treat effects as an interface to the outside world and perform effects as if you were calling system calls in other languages.
In other words, if you can resolve the effect in the language, use the "handler in the language", and if you want to cause real side effects, use the "handler outside the language".

**Note**
> By the way, the type system can enumerate all system calls statically. Isn't it a good idea for security?

## Appendix 1. Higher-order functions and effects

How to type a higher-order function, such as `map`?

Here is the type of `map`:

$$\texttt{'forall a} \ $$

$$\texttt{'forall b} \ $$

$$\texttt{'forall t:} \
\backslash \texttt{a} \ \rightarrow \ \texttt{b} $$

$$\backslash \texttt{t} \ \rightarrow \
\backslash \texttt{[a]} \ \rightarrow \
\texttt{!} \ \^{} \ \texttt{t(a) [b]}$$

I guess you can read the rest except for `^t(a)`.
It means the set of effects that may occur when the `t` function is called with a parameter type, `a`.

The compiler can correctly infer the type of the result of a `map` call even if you pass an effectful function to it.

With prefix `!` you can put an `effect expr`, such as `{effect, ...}` described earlier, to a type. There are also `^t(a)` (effect set for function call), `-<effect expr, effect expr>` (subtraction of effect sets), and `+<effect expr, ...>` (union of effect sets) other than `{effect, ...}`, so the compiler can type high-order functions correctly[^needsproof].

**Note**
> As for the implementation, there are not enough test cases and implementations here, so I would be happy if someone could help me.

## Appendix 2. Effect inference

Expressions containing effectful expressions are typed as follows (where `e` denotes a set of effects):

$$\texttt{!} \ e \ \tau$$

When the body of the function is $\texttt{!} \ e \ \tau$, the type of the function is $\backslash \ \tau_1 \ \rightarrow \ \texttt{!} \ e \ \tau$.

For `handle` expressions, the set of effects of the target expression is $e_{target}$, the set of effects handled is $e_{handled}$, and for each handler, the set of effects of `handler1` is $e_{handler1}$, and the set of continuation effects in `handler1` is $e_{continue1}$, then the set of effects of the `handle` expression is as follows.

$$\texttt{let} \ e_{remain} = \\\\ e_{target} - e_{handled};$$

$$\texttt{let} \ e_{handler1remain} = e_{handler1} - e_{continue1};$$

$$\texttt{let} \ e_{handler2remain} = e_{handler2} - e_{continue2};$$

$$e_{remain} + e_{handler1remain} + e_{handler2remain} + ...$$

For details, please read the [implementation of typeinfer](https://github.com/Hihaheho/Desk/tree/main/crates/systems).

----

Most of the sentences are translated from Japanese with https://www.DeepL.com/Translator.

[^overreacted]: https://overreacted.io/ja/algebraic-effects-for-the-rest-of-us/
[^criticized]: This may be criticized.
[^needsproof]: I want to prove the soundness and completeness of the type system, but I don't have enough time to do it.

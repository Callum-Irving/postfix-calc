# Postfix Math Parser

## What is postfix notation?

Postfix is just a different way of writing math. It is sometimes called "reverse
polish notation". There is a good wikipedia article on it [here](https://en.wikipedia.org/wiki/Reverse_Polish_notation).

## Usage

The program can be run using `cargo run`.

Just pass a postfix expression and the result will be printed on the next line.

Example adding the numbers one and two:

```
$ cargo run
> 1 2 +
3
```

In infix (normal) notation, this looks like "1 + 2".

More complicated expression:

```
$ cargo run
> 1 2 + 3 + 6 *
36
```

In infix notation, this is "((1 + 2) + 3) * 6".

To close the program, you can type `exit` or just use Ctrl-C.

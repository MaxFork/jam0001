# Repl

This is a script programming language with **Repl** environment, This has some features and the ability to execute external commands and use the output.

By the way, You are able to write **useful commands** inside a comment and run that. So as you see the comments are not useless in this **syntax**.

## Example

```
if 5 < 3 { 
    # > sudo shutdown
}
else {
    # > echo "hi"
    # comment
}
```

## Using

```
$ cargo run
```

## Steps

- [x] Lexer
- [x] Parser
- [x] AST
- [ ] Interpreter or Generator

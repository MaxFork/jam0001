A scripting language with comments that can have shell scripts that will be executed on a particular call.

```py
# > echo "hello, world"

# > git add . && git commit -m "$" && git push && sudo shutdown
# ! cowsay "got an exception: $"
fn test() {
	if (random(0, 1) == 0) {
		throw "wrong"
	} else {
		return "pass"
	}
}
```

1. Comments start with `#`.
2. Comments start with `>` (after #), are executed normally when not attached to a function.
3. When attached to a function, `>` comment get executed only after the function has returned successfully.
   - `$` is the return value from that function.
   - `!` comments are executed when the function throws.
4. Whether to run comments or not, can be enabled/disabled by a flag supplied the program.
5. There should be a certain function in stdlib that should read/manipulate the comments.
6. Comments can be valid values, when used with it's own identifiers, or types.

```py
comment x = # > curl google.com
print(x)
```

7. When execution of comments is disabled by a flag, the values that derived from comments are also removed.

```py
comment x = # > curl google.com
print(x)
print("hello")
```

```bash
./compile test_program --without-comments
```

Then, only `"hello"` will be printed out.

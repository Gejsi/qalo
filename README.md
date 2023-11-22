# Qalo

A small toy language with some pretty cool features.

There isn't a REPL, instead, the sources are executed by reading `.ql` files (take a look at the `examples` folder).

In order to play around with it, pass the wanted file paths as arguments:

```console
cargo run -- examples/map.ql examples/reduce.ql
```

# Features

Qalo is heavily focused on using expressions, rather than statements:
for example, functions are simply closures binded through a `let` statement.

Notably, Qalo offers first-class functions, arrays, hash maps, built-in functions
and several features commonly found in programming languages.

## Statements

**`let` statements** bind an identifier to the current environment.
Shadowing is allowed.

```
let foo = 2;

{
  let foo = 3;
  println(foo); // => 3
}

println(foo); // => 2
```

**`return` statements** stop the evaluation of the most outer block and return its expression.
They cannot be used at the program-level, only inside other blocks.

```
let add = fn(x, y) {
  return x + y;
};
```

**Assignment statements** allow to re-bind any identifier.

```
let foo = 1;
foo = foo + 1;
println(foo); // => 2
```

**Expression statements** represent expressions used in a place where statements are expected.
The important thing to notice is that their evaluation result isn't discarded,
meaning that the last evaluated expression will be the result of the entire block (a-la-Rust). The semicolon at the end is optional.

```
// common side-effect usage, e.g. a call expression where a function is invoked.
println("Hello world!");

// avoid `return` statements
let add = fn(x, y) {
  x + y
};
println(add(1, 2)); // => 3
```

## Expressions

Most of the math-related stuff commonly found in other programming languages is supported.

```
1 + 1 * 2 % (3 / 4)

true != false

2 > 1 || 3 <= 4 && foo[0] == !true
```

Prefix operators: `!`, `-`.

Infix operators: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`.

Postfix operators: `[]`, `()`.

### Strings

Strings concatenation is allowed. Escape characters aren't supported.

```
let foo = "Hello";
let bar = "world!";
println(foo + " " + bar); // => "Hello world!"
```

### If-else

Typical if-else, but remember this is an expression! So, things like this are allowed:

```
if condition {
  println("Yes");
} else {
  println("No");
}

let foo = if 100 > 0 { 2 } else { -1 };
println(foo); // => 2
```

`else if` blocks after the `if` aren't supported.

### Functions

Functions have this syntax:

```
fn(param) { body }
```

Qalo functions are treated as first-class citizens, so functions can:

1. Be passed as parameters to other functions.
2. Return other functions.

```
// 1. Functions as arguments
let add = fn(x, y) { x + y };
let sub = fn(x, y) { x - y };
let foo = fn(a, b, callback) { callback(a, b) };
println(foo(2, 2, add)); // => 4
println(foo(10, 2, sub)); // => 8

// 2. Return functions
let makeAdder = fn() {
  let add = fn(x, y) { x + y };
  add;
};
let adder = makeAdder();
let result = adder(1, 2);
println(result); // => 3
```

Functions in Qalo are closures, so they are evaluated within the environment they were created. Closures are really useful, as they let you encapsulate data and operate on it.

### Arrays

Arrays are ordered lists of elements. In Qalo, the elements inside the arrays can be any type of expression.

```
let arr = ["Foo", 28 + 1, fn(x) { x * x }, [100, 300]];
println(arr[0])    // => "Foo";
println(arr[1])    // => 29;
println(arr[2](3)) // => 9;
println(arr[3])    // => [100, 300];
```

### Hash Maps

Data structure that maps keys to values. Currently, only strings can be used as keys.

```
let map = { "foo": 1 + 1, "bar": fn(x) { x * x } };
println(map["foo"])    // => 2;
println(map["bar"](3)) // => 9;
```

## Built-in functions

Qalo offers some functions that don't need to be defined by the user,
as they are implemented into the language itself (like `make()` in Go).
Built-in functions have the precedence over user-defined functions with the same name.

### `len(param)`

`len` returns the length the string/array that it receives as argument.

```
let str = len("Hello");
let arr = len([100, 200]);
println(str); // => 5
println(arr); // => 2
```

### `append(array, ...elements)`

`append` pushes all the variadic elements to the array specified as the first argument.
`append` doesn't modify the original array, it returns an **updated copy**.

```
let arr = [1, 2, 3];
let new = append(arr, 100, 200);
println(new) // => [1, 2, 3, 100, 200];
```

### `rest(array)`

`rest` returns a new array containing all elements of the array passed as argument, **except the first one**.

```
let arr = [1, 2, 3, 4];
println(rest(arr)) // => [2, 3, 4]
```

### `println(...elements)`

Prints to the standard output, **with** a newline.

### `print(...elements)`

Prints to the standard output, **without** a newline.

# Usage

Here is a `map` function written in Qalo:

```
let map = fn(arr, f) {
  let iter = fn(arr, accumulated) {
    if len(arr) == 0 {
      accumulated
    } else {
        iter(rest(arr), append(accumulated, f(arr[0])));
    }
  };

  iter(arr, []);
};

let arr = [1, 2, 3, 4];
let double = fn(x) { x * 2 };
println(map(arr, double)); // => [2, 4, 6, 8]
```

While a `reduce` can be written like this:

```
let reduce = fn(arr, initial, f) {
  let iter = fn(arr, result) {
    if len(arr) == 0 {
      result
    } else {
      iter(rest(arr), f(result, arr[0]));
    }
  };

  iter(arr, initial);
};

let sum = fn(arr) {
  return reduce(arr, 0, fn(initial, el) { initial + el });
};

println(sum([1, 2, 3, 4, 5])); // => 15
```

# Extra

Qalo was inspired by the book _Writing an interpreter in Go_ by Thorsten Ball.

It doesn't have:

- Garbage collection.
- Support for most types of numbers. Only `int32`s are supported.
- Performance feats. Qalo is slow.
- Comments.
- Different types of binding statements (`let`, `var`...).
- `while`/`for` loops.

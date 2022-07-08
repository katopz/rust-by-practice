# Variables

### Binding and mutability

1. 🌟 A variable can be used only if it has been initialized.

```rust,editable

// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
```

<script>let answers_1 = [[3,14," = 5"]]</script>

<button class="hint" onclick="this.solveAt(...answers_1)">💡 HINT</button>

2. 🌟 Use `mut` to mark a variable as mutable.

```rust,editable

// Fill the blanks in the code to make it compile
fn main() {
    let __ = 1;
    __ += 2;

    assert_eq!(x, 3);
    println!("Success!");
}
```

<script>let answers_2 = ["mut x","x"]</script>

<button class="hint" onclick="this.solveUnder(...answers_2)">💡 HINT</button>

### Scope

A scope is the range within the program for which the item is valid.

3. 🌟

```rust,editable

// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}
```

<script>let answers_3 = `
// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {}", x);
}
`</script>

<button class="hint" onclick="this.solveAll(answers_3)">💡 HINT</button>

<!-- TODO: REPLACE -->

4. 🌟🌟

```rust,editable

// Fix the error with the use of define_x
fn main() {
    println!("{}, world", x);
}

fn define_x() {
    let x = "hello";
}
```

<script>let answers_4 = `
// Fix the error with the use of define_x
fn main() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> String {
    let x = "hello".to_string();
    x
}
`</script>

<button class="hint" onclick="this.solveAll(answers_4)">💡 HINT</button>

<!-- TODO: REPLACE -->
<!-- TODO: MULTIPLE HINTS -->

### Shadowing

You can declare a new variable with the same name as a previous variable, here we can say \*\*the first one is shadowed by the second one.

5. 🌟🌟

```rust,editable

// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x = 42;
    println!("{}", x); // Prints "42".
}
```

<script>let answers_5 = `
// Only modify \`assert_eq!\` to make the \`println!\` work(print \`42\` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}
`</script>

<button class="hint" onclick="this.solveAll(answers_5)">💡 HINT</button>

<!-- TODO: REPLACE -->

6. 🌟🌟

```rust,editable

// Remove a line in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x;
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}
```

<script>let answers_6 = [[7,4,"// "]]</script>

<button class="hint" onclick="this.solveAt(...answers_6)">💡 HINT</button>

### Unused variables

7. 🌟 Fix the warning below

> Note: none of the solutions is to remove the line `let x = 1`

```rust,editable

fn main() {
    let x = 1;
}

// Warning: unused variable: `x`
```

<script>let answers_7 = [[2,8,"_"]]</script>

<button class="hint" onclick="this.solveAt(...answers_7)">💡 HINT</button>

<!-- TODO: MULTIPLE HINTS -->

### Destructuring

8. 🌟🌟 We can use a pattern with `let` to destructure a tuple to separate variables.

> Tips: you can use Shadowing or Mutability

```rust,editable

// Fix the error below with least amount of modification
fn main() {
    let (x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
```

<script>let answers_8 = [[3,9,"mut "]]</script>

<button class="hint" onclick="this.solveAt(...answers_8)">💡 HINT</button>

<!-- TODO: MULTIPLE HINTS -->

### Destructuring assignments

Introduced in Rust 1.59: You can now use tuple, slice, and struct patterns as the left-hand side of an assignment.

9. 🌟🌟

> Note: the feature `Destructuring assignments` need 1.59 or higher Rust version

```rust,editable

fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], __);

    println!("Success!");
}
```

<script>let answers_9 = ["[3, 2]"]</script>

<button class="hint" onclick="this.solveUnder(...answers_9)">💡 HINT</button>

> You can find the solutions [here](https://github.com/sunface/rust-by-practice)(under the solutions path), but only use it when you need it

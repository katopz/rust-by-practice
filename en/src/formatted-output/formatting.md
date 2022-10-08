# formating

## Positional arguments

1.🌟🌟
```rust,editable
/* Fill in the blanks */
fn main() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
    assert_eq!(format!("{1}{0}", 1, 2), __);
    assert_eq!(format!(__, 1, 2), "2112");
    println!("Success!");
}
```

{{#playground formatting_1_0.rs answer}}



## Named arguments

2.🌟🌟
```rust,editable
fn main() {
    println!("{argument}", argument = "test"); // => "test"

    /* Fill in the blanks */
    assert_eq!(format!("{name}{}", 1, __), "21");
    assert_eq!(format!(__, a = "a", b = 'b', c = 3), "a 3 b");

    /* Fix the error */
    // named argument must be placed after other arguments
    println!("{abc} {1}", abc = "def", 2);

    println!("Success!")
}
```

{{#playground formatting_2_0.rs answer}}



## Padding with string

3.🌟🌟 By default, you can pad string with spaces
```rust,editable
fn main() {
    // the following two are padding with 5 spaces
    println!("Hello {:5}!", "x"); // =>  "Hello x    !"
    println!("Hello {:1$}!", "x", 5); // =>  "Hello x    !"

    /* Fill in the blanks */
    assert_eq!(format!("Hello __!", 5, "x"), "Hello x    !");
    assert_eq!(format!("Hello __!", "x", width = 5), "Hello x    !");

    println!("Success!")
}
```

{{#playground formatting_3_0.rs answer}}



4.🌟🌟🌟 Left align, right align, pad with specified characters.
```rust,editable
fn main() {
    // left align
    println!("Hello {:<5}!", "x"); /// => Hello x    !
    // right align
    assert_eq!(format!("Hello __!", "x"), "Hello     x!");
    // center align
    assert_eq!(format!("Hello __!", "x"), "Hello   x  !");

    // left align, pad with '&'
    assert_eq!(format!("Hello {:&<5}!", "x"), __);

    println!("Success!")
}
```

{{#playground formatting_4_0.rs answer}}



5.🌟🌟 You can pad numbers with extra zeros.
```rust,editable
fn main() {
    println!("Hello {:5}!", 5); // => Hello     5!
    println!("Hello {:+}!", 5); // =>  Hello +5!
    println!("Hello {:05}!", 5); // => Hello 00005!
    println!("Hello {:05}!", -5); // => Hello -0005!

    /* Fill in the blank */
    assert!(format!("{number:0>width$}", number = 1, width = 6) == __);

    println!("Success!")
}
```

{{#playground formatting_5_0.rs answer}}



## precision
6.🌟🌟 Floating point precision
```rust,editable
/* Fill in the blanks */
fn main() {
    let v = 3.1415926;

    println!("{:.1$}", v, 4); // same as {:.4} => 3.1416

    assert_eq!(format!("__", v), "3.14");
    assert_eq!(format!("__", v), "+3.14");
    assert_eq!(format!("__", v), "3");

    println!("Success!")
}
```

{{#playground formatting_6_0.rs answer}}



7.🌟🌟🌟 string length
```rust,editable
fn main() {
    let s = "Hello, world!";

    println!("{0:.5}", s); // => Hello

    assert_eq!(format!("Hello __!", 3, "abcdefg"), "Hello abc!");

    println!("Success!")
}
```   

## binary, octal, hex

- format!("{}", foo) -> "3735928559"
- format!("0x{:X}", foo) -> "0xDEADBEEF"
- format!("0o{:o}", foo) -> "0o33653337357"
  
8.🌟🌟
```rust,editable
fn main() {
    assert_eq!(format!("__", 27), "0b11011");
    assert_eq!(format!("__", 27), "0o33");
    assert_eq!(format!("__", 27), "0x1b");
    assert_eq!(format!("__", 27), "0x1B");

    println!("{:x}!", 27); // hex with no prefix => 1b

    println!("{:#010b}", 27); // pad binary with 0, width = 10,  => 0b00011011

    println!("Success!")
}
```

{{#playground formatting_7_0.rs answer}}



## Capture the environments
9.🌟🌟🌟
```rust,editable
fn get_person() -> String {
    String::from("sunface")
}

fn get_format() -> (usize, usize) {
    (4, 1)
}

fn main() {
    let person = get_person();
    println!("Hello, {person}!");

    let (width, precision) = get_format();
    let scores = [("sunface", 99.12), ("jack", 60.34)];
    /* Make it print:
    sunface:   99.1
    jack:   60.3
    */
    for (name, score) in scores {
        println!("{name}: __");
    }
}
```

{{#playground formatting_9_0.rs answer}}




## Others

**Example**
```rust,editable
fn main() {
    // exponent
    println!("{:2e}", 1000000000); // => 1e9
    println!("{:2E}", 1000000000); // => 1E9

    // pointer address
    let v= vec![1, 2, 3];
    println!("{:p}", v.as_ptr()); // => 0x600002324050

    // escape
    println!("Hello {{}}"); // => Hello {}
}
```

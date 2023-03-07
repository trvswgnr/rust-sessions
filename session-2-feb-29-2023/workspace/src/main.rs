fn main() {
    let_else(Some(5));
    inclusive_range(5);
    multi_threading();
}

/// 1. `let-else`
/// This feature was stabilized in Rust 1.65.0 and simplifies some very common
/// error handling patterns. It is the natural counterpart to the `if-let`,
/// just as `else` is the natural counterpart to `if`.
fn let_else(x: Option<i64>) -> i64 {
    // Without let-else, we would have to write:
    // if let Some(val) = x {
    //     if val == 5 {
    //         println!("it's five");
    //     } else {
    //         println!("not five");
    //     }
    // } else {
    //     println!("x was None, exiting...");
    //     return;
    // }

    // With let-else, we can write:
    let Some(mut val) = x else {
        println!("x was None, exiting...");
        // this returns from the function, and requires a diverging expression
        // such as a return, break, continue, or panic
        return -1;
    };

    // This is a counterpart to if let expressions, and the pattern matching
    // works identically, except that the value from the pattern match is
    // assigned to the surrounding scope rather than the block's scope.
    if val == 5 {
        println!("it's five");
    } else {
        println!("not five");
        val *= val;
    }

    //* this:
    // let Some(val) = x else { return -1; };

    //* is equivalent to this:
    // let val = match x {
    //     Some(thing) => thing,
    //     _ => {
    //         return -1;
    //     }
    // };

    // The RFC contains a lot of other patterns that might be implemented
    // with let-else, so keep an eye out for them!
    // @see https://rust-lang.github.io/rfcs/3137-let-else.html
    val
}

/// 2. `..=` in pattern matching
///
/// The previous `...` syntax for ranges has been deprecated as of Rust 1.66.0
/// in favor of the `..=` syntax. This syntax is now supported in pattern matching.
///
/// @note The exlusive range (`..`) syntax is stil experimental and currently
/// requires the `#![feature(exclusive_range_pattern)]` attribute.
///
/// @see https://github.com/rust-lang/rust/releases/tag/1.66.0
fn inclusive_range(x: i32) {
    match x {
        // the exclusive range syntax is still supported
        // but now we can use the inclusive range syntax
        1..=5 => println!("x is between 1 and 5"),
        _ => println!("x is not between 1 and 5 (including 1 and 5)"),
    }
}

/// 3. Lifetime elision
///
/// Lifetime elision is a feature that allows the compiler to infer the lifetime
/// of a reference based on the context in which it is used. This is a very
/// powerful feature that allows us to write less code, but it can also be
/// confusing to understand.
///
///
/// The compiler will attempt to apply three rules to infer the lifetime of a
/// reference. If after the rules are applied and there are still references
/// that the compiler cannot infer the lifetime of, it will throw an error.
///
/// The rules are as follows:
/// 1. Each parameter that is a reference gets its own lifetime parameter.
/// 2. If there is exactly one input lifetime parameter, that lifetime is
///    assigned to all output lifetime parameters.
/// 3. If there are multiple input lifetime parameters, but one of them is
///    `&self` or `&mut self` because this is a method, the lifetime of `self`
///    is assigned to all output lifetime parameters.
///
/// @see https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
/// @see https://doc.rust-lang.org/reference/lifetime-elision.html
use std::ffi::CStr;
use std::process::Command;
trait ToCStr {
    fn to_c_str(&self) -> &CStr;
}

struct Thing<'a> {
    buf: &'a mut [u8],
}

trait DoesTheThing {}

trait LifetimeElision {
    fn do_thing_elided(s: &str); // elided
    fn do_thing_elided_2(s: &'_ str); // also elided
    fn do_thing_expanded<'a>(s: &'a str); // expanded

    fn debug1(lvl: usize, s: &str); // elided
    fn debug2<'a>(lvl: usize, s: &'a str); // expanded (note usize does not get a lifetime, because it is not a reference)

    fn substr1(s: &str, until: usize) -> &str; // elided
    fn substr2<'a>(s: &'a str, until: usize) -> &'a str; // expanded

    fn get_mut1(&mut self) -> &mut dyn DoesTheThing; // elided
    fn get_mut2<'a>(&'a mut self) -> &'a mut dyn DoesTheThing; // expanded

    fn args1<T: ToCStr>(&mut self, args: &[T]) -> &mut Command; // elided
    fn args2<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded (output gets lifetime of self)

    fn new1(buf: &mut [u8]) -> Thing<'_>; // elided - preferred since struct has a lifetime specifier
    fn new2(buf: &mut [u8]) -> Thing; // elided
    fn new3<'a>(buf: &'a mut [u8]) -> Thing<'a>; // expanded
}
// types also get lifetime elision if they refer to functions that have references as parameters
type FunPtr1 = fn(&str) -> &str; // elided
type FunPtr2 = for<'a> fn(&'a str) -> &'a str; // expanded

type FunTrait1 = dyn Fn(&str) -> &str; // elided
type FunTrait2 = dyn for<'a> Fn(&'a str) -> &'a str; // expanded

// when the compiler cannot infer the lifetime of a reference, it will throw an error, so we would
// have to specify the lifetime of the reference ourselves. For example, the following code will
// throw an error:
// fn print1(s: &str, x: &str) -> &str {
//     println!("{}", s);
//     // check if x contains s
//     if x.contains(s) {
//         x
//     } else {
//         s
//     }
// }

// the compiler applies 'a and 'b to the references in the function signature, but it doesn't
// know which lifetime to apply to the return type, so it throws an error.
// we can fix this by specifying the lifetime of the references ourselves:
fn print2<'a>(s: &'a str, x: &'a str) -> &'a str {
    println!("{}", s);
    // check if x contains s
    if x.contains(s) {
        x
    } else {
        s
    }
}

/// 4. Number types
///
/// Rust has a number of different number types, and it can be confusing to know
/// which one to use when.
///
/// In general, the `i` prefix is used for signed integers (can hold both positive
/// and negative numbers) and the `u` prefix is used for unsigned integers (can
/// only hold positive numbers).
///
/// The number after the prefix is the number of bits that the number type can
/// hold. For example, `i8` is an 8-bit signed integer, and `u16` is a 16-bit
/// unsigned integer.
///
/// Aside from some corner cases, smaller number types use less memory than
/// larger number types.
fn number_types() {
    // Let's start with the smallest number type, the `u8` type.
    // This type is an unsigned integer that can hold values from 0 to 2^8-1 (0 to 255).
    // `u8` can be useful for storing things like colors, where each color
    // channel is represented by a number from 0 to 255. Age is also a good
    // example of a number that can be represented by a `u8` type.
    // In this case, using a `usize` type would be overkill and take 8 bytes
    // instead of 1 byte with a `u8` type (assuming that we are on 64-bit architecture).
    let r: u8 = 255;
    let g: u8 = 200;
    let b: u8 = 50;
    let age: u8 = 30;
    println!("r: {r}, g: {g}, b: {b}, age: {age}");

    // The `u8` type is an unsigned integer, which means that it can only hold
    // positive numbers. If we try to assign a negative number to a `u8` type,
    // the compiler will throw an error.
    // let color: u8 = -1; // error: expected `u8`, found `i32`

    // The `i8` type is a signed integer, which means that it can hold both
    // positive and negative numbers. The range of values that it can hold is
    // from -(2^7) to 2^7-1 (-128 to 127). This type is useful for storing
    // numbers that aren't very large, but that can be negative, like temperatures.
    let temp: i8 = -10;
    println!("temp: {temp}");

    // From here on the only difference between the `u` and `i` types is the
    // number of bits that they can hold. The `u16` type is an unsigned integer
    // that can hold values from 0 to 2^16-1 (0 to 65535). The `i16` type is a
    // signed integer that can hold values from -(2^15) to 2^15-1 (-32768 to 32767).
    // As a rule of thumb, the `u` types can hold twice as many values as the
    // `i` types.
    let x: u16 = 65535;
    let y: i16 = -32768;
    // let y: i16 = -32769; // whoops, this is out of range
    println!("x: {x}, y: {y}");

    // When we declare an integer type without specifying the number of bits,
    // the compiler will use the default type, which is `i32`. The `i32` type
    // can hold values from -(2^31) to 2^31-1 (-2,147,483,648 to 2,147,483,647 or approx. -2.1 billion to 2.1 billion).
    let z = 123; // default type is i32

    // The largest unsigned integer type is `u128`, which can hold values from
    // 0 to 2^128-1 (0 to approx 340 undecillion).

    // The largest signed integer type is `i128`, which can hold values from
    // -(2^127) to 2^127-1 (approx -170 undecillion to 170 undecillion).

    // Floating point types
    // Rust also has two floating point types: `f32` and `f64`. The `f32` type is
    // a single-precision floating point type, and the `f64` type is a
    // double-precision floating point type. "Single-precision" just means that
    // the number is represented by less bits than a "double-precision" number.
    //
    // Rust's floating point types are based on the IEEE 754-2008 standard, which
    // means that f32 can hold a number from 1.17549435e-38 to 3.40282347e+38,

    // `usize` and `isize`
    // The `usize` and `isize` types depend on the architecture of the computer
    // that the program is running on. For example, if the program is running on
    // a 64-bit architecture, then `usize` and `isize` will be 64 bits. If the
    // program is running on a 32-bit architecture, then `usize` and `isize` will
    // be 32 bits.
    //
    // The `usize` type is used for indexing collections, like arrays and vectors.
    // The `isize` type is used for indexing collections that can contain negative
    // numbers, like slices.
}

/// 5. Multi-threading
/// Rust has a built-in multi-threading library that makes it easy to create
/// and manage threads.
/// The `std::thread` module contains the `spawn` function, which is used to
/// create a new thread.
/// The `spawn` function takes a closure as an argument, and returns a `JoinHandle`.
/// The `JoinHandle` is a type that represents a thread that has been spawned.
/// The `JoinHandle` has a `join` method that can be used to wait for the thread
/// to finish executing.
fn multi_threading() {
    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();

    let handle_1 = std::thread::spawn(|| {
        // sleep for 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Hello from handle_1 thread!");
    });

    let handle_2 = std::thread::spawn(|| {
        println!("Hello from handle_2 thread!");
    });

    handles.push(handle_1);
    handles.push(handle_2);

    for handle in handles {
        handle.join().unwrap();
    }

    // The `move` keyword is used to move the closure's captured variables into
    // the closure. This is required because the closure is being executed in a
    // separate thread, and the variables that it is capturing are not available
    // in the new thread.
    let mut counter = 0;
    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
    let handle_3 = std::thread::spawn(move || {
        counter += 1;
        // wait for 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("counter: {}", counter);
    });
    let handle_4 = std::thread::spawn(move || {
        println!("counter: {}", counter);
    });

    handles.push(handle_3);
    handles.push(handle_4);

    for handle in handles {
        handle.join().unwrap();
    }

    // the value of `counter` at this point will be 0.
}

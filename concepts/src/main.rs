// fn main() {
//     let x = 5;

//     let x = x + 1;

//     { // No x yet (in current scope, the stacked x is present)
//         let x = x * 2; // X is initialised in heap
//         println!("The value of x in the inner scope is: {x}"); //Here x is in scope so it is stored
//     } // Here the memory is cleared hence x is gone.

//     {
//         let s = String::from("hello"); // s is valid from this point forward

//         // do stuff with s
//     }                                  // this scope is now over, and s is no
//                                        // longer valid

//     println!("The value of x is: {x}");
// }


// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // Because i32 implements the Copy trait,
//                                     // x does NOT move into the function,
//                                     // so it's okay to use x afterward.

// } // Here, x goes out of scope, then s. However, because s's value was moved,
//   // nothing special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s1 = gives_ownership();        // gives_ownership moves its return
//                                        // value into s1

//     let s2 = String::from("hello");    // s2 comes into scope

//     let s3 = takes_and_gives_back(s2); // s2 is moved into
//                                        // takes_and_gives_back, which also
//                                        // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {       // gives_ownership will move its (-> means return)
//                                        // return value into the function
//                                        // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                        // some_string is returned and
//                                        // moves out to the calling
//                                        // function
// }

// // This function takes a String and returns a String.
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string comes into
//     // scope

//     a_string  // a_string is returned and moves out to the calling function
// }



//  //OWNERSHIP
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}.");
// }

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because s does not have ownership of what
//   // it refers to, the String is not dropped.


// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s=String::from("Hello!!");

//     let p= &mut s;
//     let r= &mut s;
//     println!("Printing {p} and {r}");
// }          // cannot borrow `s` as mutable more than once at a time
//             // --> src/main.rs:108:12
//             //  |
//             // 107 |     let p= &mut s;
//             //  |            ------ first mutable borrow occurs here
//             // 108 |     let r= &mut s;
//             //  |            ^^^^^^ second mutable borrow occurs here
//             // 109 |     println!("Printing {p} and {r}");
//             //  |                         - first borrow later used here


//SLICE

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}


// Original function
// // Should take ownership
// fn string_uppercase(mut data: &String) {
//     data = &data.to_uppercase();

//     println!("{}", data);
// }

// - `mut` can have different meanings depending on where it is placed (like C++!)
//   - ex. mut data: &mut String
//     - The first `mut` refers to the fact that the binding (i.e. name) is mutable.  If only that mut was present, then we could change the value that the reference is pointing to, but not the reference itself
//       - Note that & (and * as well) are low precedent, meaning that in line 31, the & applies after the to_uppercase
//     - The second `mut` refers to the fact that the underlying memory at the reference is mutable.  If only that mut was present, then we could change the variable at the location (i.e. append to 'data') but not where it points to

// In this original code, we intend for the reference `data` to always last at least as long as it's being used (AKA for all of string_uppercase)
//  - An issue is that data.to_uppercase() creates a new string variable, whose ownership is transferred on return.  However, since it is never moved to a concrete variable and instead 'stays' as a temporary value
// There is no direct equivalent of this behaviour in C++ (except maybe returning a heap-allocated object)
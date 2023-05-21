// strings4.rs

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

// String vs str (commonly &str) in rust (vs C++)
// String <-> std::string
    // Like std::string.  In C++, memory is allocated on the heap and automatically managed by the lifetime of the string object.  Contains all of the bells and whistles you'd normally expect anyway.  In rust, the only major difference is that lifetime is managed in the Rusty way, but the object still owns the underlying string
// &str <-> char*
    // Can correspond to an 'owned' string, but is often (in rust) just a view into a string owned by somebody else.  These strings have a couple more functions though, and use UTF8 by default

// Note that different functions return &str and others return String.  This is due to optimization reasons.  Some string functions (like trim) have return values that will always substring the original string, so those can be returned as a reference for efficiency reasons, while other functions (like replace) destructively modify the string, and thus must be copied and the new string must have its own ownership properties
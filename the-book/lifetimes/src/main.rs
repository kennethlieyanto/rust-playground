use std::fmt::Display;

fn main() {
    let string1 = String::from("abcde");
    {
        let string2 = String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());

        let announcement = String::from("WHADDUP");
        let another_result =
            longest_with_an_announcent(string1.as_str(), string2.as_str(), announcement);
        // println!("The longest string is {another_result}");
    }
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }
//
fn longest_with_an_announcent<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

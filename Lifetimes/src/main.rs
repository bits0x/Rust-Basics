use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let res: &str;
    {
        /*
            string2 lifetime is in this block only, so it will give the below error
            error: `string2` does not live long enough
            label: borrowed value does not live long enough
         */
        let string2 = String::from("xyz");  
        res = longest(&string1.as_str(), &string2.as_str());
    }

    println!("Longest String is {}", res);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // return value have the lifetime of the smallest lifetime of the variables
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T) -> &'a str 
where T: Display 
{
    println!("Announcement = {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; //problem as mut
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // let nul_ref = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");
    // s no longer exists outside the scope of this method
    &s;
    "a".to_string()
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

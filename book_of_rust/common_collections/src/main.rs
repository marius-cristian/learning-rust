fn main() {
    // _vectors();
    // _strings();
    _hashmaps();
}

fn _hashmaps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    //  value moved here
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{:?}", field_value);
    // value borrowed here after move

    println!("{:?}", scores2.get(&String::from("Blue")));
    println!("{:?}", scores2.get("Yellow"));
    println!("{:?}", scores2.get(&String::from("Blve")));

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Yellow")).or_insert(5);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Blve")).or_insert(50);
    println!("{:?}", scores);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn _strings() {
    let mut v: Vec<String> = Vec::new();
    let data = "initial السلام عليكم contents 1";

    let s = data.to_string();
    println!("{:?}", s);
    // the method also works on a literal directly:
    let s = "initial नमस्ते contents 2".to_string();
    println!("{:?}", s);

    let mut s = String::from("initial 你好 contents 3");
    println!("{:?}", s);

    v.push(String::from(""));
    v.push(String::from("السلام عليكم"));
    v.push(String::from("Dobrý den"));
    v.push(String::from("Hello"));
    v.push(String::from("שָׁלוֹם"));
    v.push(String::from("नमस्ते"));
    v.push(String::from("こんにちは"));
    v.push(String::from("안녕하세요"));
    v.push(String::from("你好"));
    v.push(String::from("Olá"));
    v.push(String::from("Здравствуйте"));
    v.push(String::from("Hola"));
    s.push_str(" 1");
    s.push_str(" 2");
    s.push_str(" 3");
    s.push_str(" \t \n");
    v.push(s);
    for i in v {
        println!("{:?}", i);
    }

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    println!("{:?}", s1);
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{:?} {}", s2, s3);
    let s4 = format!("{}-{}", s2, s3);
    println!("{:?}", s4);

    let hello = "Здравствуйте";
    // let sh = hello[0]  -- cant index String
    let sh = &hello[0..4]; // but can slice str
                           // but cant slice half a UTF-8 char
                           // &hello[0]
    println!("{:?}", sh);

    for c in "नमस्ते".chars() {
        println!("{}", c);
        // on ubuntu terminal stdout
        // the diacritics are not displayed by themselves
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

fn _vectors() {
    let mut v: Vec<i32> = Vec::new();
    let mut v_macro = vec![1, 2, 3, 4];
    v.push(1);
    v_macro.push(10);
    let fifth: &i32 = &v_macro[4];
    println!("fifth element is {}", fifth);
    match v_macro.get(5) {
        Some(val) => println!("found {:?}", val),
        None => println!("PANIC "),
    }
    for i in &v {
        println!("{}", i);
    }

    for i in &v_macro {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
        println!("i is {:?}", i);
    }
    for i in &v {
        println!("increased val: {}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}

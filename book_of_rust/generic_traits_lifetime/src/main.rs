// generic data types

fn largest_initial(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// traits
use generic_traits_lifetime::{
    self, notify2, returns_summarizable, NewsArticle, Pair, Summary, Tweet,
};

fn main() {
    // generic data types
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_initial(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    println!(
        "Largest int and char {:?} {:?}",
        largest(&number_list),
        largest(&char_list)
    );

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    let _integer_and_float = Point { x: 5, y: 4.0 };
    println!(
        "p.x = {}, p.y = {:.2}",
        _integer_and_float.x(),
        _integer_and_float.y()
    );
    println!("{:?}", _float.distance_from_origin());
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // traits
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify2(&article, &tweet);

    returns_summarizable();

    let pr1 = Pair::new(1, 1);
    let pr2 = Pair::new(1, 2);
    pr1.cmp_display();
    pr2.cmp_display();

    let s = 3.to_string();

    println!("{:?}", s);
}

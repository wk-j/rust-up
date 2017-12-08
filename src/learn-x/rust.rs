fn add2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let x: i32 = 1;
    let y: i32 = 13i32;
    let f: f64 = 1.3f64;

    let implicit_x = 1;
    let implicit_y = 1.3;

    let sum = x + y + 13;
    let mut mutable = 1;
    mutable = 4;
    mutable += 2;

    let x: &str = "hello world!";
    println!("{} {}", f, x);

    let s: String = "hello world".to_string();
    let s_slice: &str = &s;

    println!("{} {}", s, s_slice);

    let four_ints: [i32; 4] = [1, 2, 3, 4];
    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    vector.push(5);

    let slice: &[i32] = &vector;
    println!("{:?} {:?}", vector, slice);

    let x: (i32, &str, f64) = (1, "hello", 3.4);

    let (a, b, c) = x;
    println!("{} {} {}", a, b, c);

    println!("{}", x.1);

    struct Point {
        x: i32,
        y: i32,
    }

    let origin: Point = Point { x: 0, y: 0 };
    struct Point2(i32, i32);

    let origin2 = Point2(0, 0);

    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    let up = Direction::Up;

    enum OptionalI32 {
        AnI32(i32),
        Nothing,
    }

    let two: OptionalI32 = OptionalI32::AnI32(2);
    let mothing = OptionalI32::Nothing;

    struct Foo<T> {
        bar: T,
    }

    enum Optional<T> {
        SomeVal(T),
        NoVal,
    }

    impl<T> Foo<T> {
        fn get_bar(self) -> T {
            self.bar
        }
    }

    let a_foo = Foo { bar: 1 };
    println!("{}", a_foo.get_bar());


    trait Frobnicate<T> {
        fn frobnicate(self) -> Option<T>;
    }

    impl<T> Frobnicate<T> for Foo<T> {
        fn frobnicate(self) -> Option<T> {
            Some(self.bar)
        }
    }

    let another_foo = Foo { bar: 1 };
    println!("{:?}", another_foo.frobnicate());

    /////////////////////////////////
    // 3. Pattern matching
    //////////////////////////////
    let foo = OptionalI32::AnI32(1);
    match foo {
        OptionalI32::AnI32(n) => println!("it's an i32: {}", n),
        OptionalI32::Nothing => println!("it's nothing!"),
    }


    struct FooBar {
        x: i32,
        y: OptionalI32,
    }
    let bar = FooBar {
        x: 15,
        y: OptionalI32::AnI32(32),
    };

    match bar {
        FooBar {
            x: 0,
            y: OptionalI32::AnI32(0),
        } => println!("The numbers are zero!"),
        FooBar {
            x: n,
            y: OptionalI32::AnI32(m),
        } if n == m =>
        {
            println!("The numbers are the same")
        }
        FooBar {
            x: n,
            y: OptionalI32::AnI32(m),
        } => println!("Different numbers: {} {}", n, m),
        FooBar {
            x: _,
            y: OptionalI32::Nothing,
        } => println!("The secon number is Nothing!"),
    }

    let array = [1, 2, 3];
    for i in array.iter() {
        println!("{}", i);
    }

    for i in 0u32..10 {
        print!("{}", i);
    }

    println!("");

    if 1 == 1 {
        println!("Matchs is working!");
    } else {
        println!("Oh no ...");
    }

    let value = if true { "good" } else { "bad" };

    while 1 == 2 {
        println!("The universe is operating normally.");
    }

    /*
    loop {
        println!("Hello!");
    }*/

    let mut mine: Box<i32> = Box::new(3);
    *mine = 5;

    let mut now_its_mime = mine;
    *now_its_mime += 2;

    println!("{}", now_its_mime);

    let mut var = 4;
    var = 3;
    let ref_var: &i32 = &var;

    println!("{}", var);
    println!("{}", *ref_var);


    let mut var2 = 4;
    let ref_var2: &mut i32 = &mut var2;
    *ref_var2 += 2;

    println!("{}", *ref_var2);
}

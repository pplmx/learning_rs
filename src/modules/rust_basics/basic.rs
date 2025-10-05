pub fn basic_num() {
    // 整型的默认类型是 i32
    // 浮点型的默认类型是 f64

    // 由于未指定类型, 编译器会根据赋值的类型进行推断, 这里是 i32
    let x = 5;

    // rust 中的变量默认是不可变的, 如果需要可变变量, 需要使用 "mut" 关键字
    // x = 10; // 由于 x 是不可变的, 这里会报错

    // 由于未指定类型, 编译器会根据赋值的类型进行推断, 这里是 f64
    let y = 10.1;

    // 指定类型
    let z: i64 = 1234567890;

    // 定义一个可变变量
    let mut k = 10;
    println!("mutable variable ->k: {}", k);
    k = 20; // 由于 k 是可变的, 这里不会报错

    // 也可以通过数值的类型后缀来指定类型
    let k0 = 10i8;
    let k1 = 10i16;
    let k2 = 10i32;
    let k3 = 10i64;
    let k4 = 10u8;
    let k5 = 10u16;
    let k6 = 10u32;
    let k7 = 10u64;
    let k8 = 10f32;
    let k9 = 10f64;

    // isize 和 usize 的长度是平台相关的整型.
    // 如果 cpu 是 32 位, 那么 isize 是 i32
    // 如果 cpu 是 64 位, 那么 isize 是 i64
    let k_isize = 10isize;
    let k_usize = 10usize;

    // 下划线可以用来增加数字的可读性, 但是不会影响数字的值
    let n0 = 1000_i64; // i64 类型的 1000
    let n1 = 1_0_0_0; // i32 类型的 1000
    let n2 = 10_000_000; // i32 类型的 10000000

    // call the add function
    let f = add(10, 20);

    // 比较浮点数
    // NOTES:
    //     1. 浮点数是不精确的, 所以我们需要比较它们之间的差值
    //     2. 我们需要显式地将 f1 或 f2 定义为 f64, 否则 abs() 函数将无法工作
    /*
    let f1 = 0.1 + 0.2;
    let f2 = 0.3;
    let is_equal = (f1 - f2).abs() < f64::EPSILON;
    It will cause an error:
        error[E0689]: can't call method `abs` on ambiguous numeric type `{float}`
    */
    let f1: f64 = 0.1 + 0.2; // Notes: 需要显式地将 f1 定义为 f64
    let f2 = 0.3;
    let is_equal = (f1 - f2).abs() < f64::EPSILON;
    if is_equal {
        println!("f1 and f2 are equal");
    } else {
        println!("f1 and f2 are not equal");
    }

    // define an array with repeating values in basic types
    let arr = [1; 5]; // [1, 1, 1, 1, 1]

    // define an array with repeating String values
    let s_arr: [String; 3] = std::array::from_fn(|_i| String::from("hello"));

    // print the variables
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("k: {}", k);
    println!("k0: {}", k0);
    println!("k1: {}", k1);
    println!("k2: {}", k2);
    println!("k3: {}", k3);
    println!("k4: {}", k4);
    println!("k5: {}", k5);
    println!("k6: {}", k6);
    println!("k7: {}", k7);
    println!("k8: {}", k8);
    println!("k9: {}", k9);
    println!("k_isize: {}", k_isize);
    println!("k_usize: {}", k_usize);
    println!("n0: {}", n0);
    println!("n1: {}", n1);
    println!("n2: {}", n2);
    println!("f: {}", f);
    println!("arr: {:?}", arr);
    println!("s_arr: {:?}", s_arr);
}

pub fn basic_others() {
    let is_active = true;
    // Rust 中的 char 不仅仅是 ASCII, 它还可以是 unicode, 比如单个中文字符, emoji 等
    let chara = 'a';
    let char_ni = '你';
    let emoji_cat = '😻';
    let emoji_cat_hex = '\u{1F63B}';
    let hi = "hello world";

    test_for();

    println!("is_active: {}", is_active);
    println!("chara: {}", chara);
    println!("char_ni: {}", char_ni);
    println!("emoji_cat: {}", emoji_cat);
    println!("emoji_cat_hex: {}", emoji_cat_hex);
    println!("hi: {}", hi);

    // 函数没有返回值, 所以返回值是 ()
    no_return();

    // 函数演示了变量的作用域和遮蔽
    variable_scope_shadowing();
}

fn add(x: i32, y: i32) -> i32 {
    // return x + y; // 通过 return 关键字返回值, 与下面的代码等价
    x + y // rust 中的函数默认返回最后一个表达式的值, 表达式不需要使用分号结尾
}

fn no_return() {
    // 其他语言中的无返回值函数, 在 Rust 中返回值是 (), 即 unit 类型. 它是一个空元组.
}

fn variable_scope_shadowing() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}

fn test_for() {
    // 如果有"="号, 则是全闭区间
    // for i in 1..5 {
    for i in 1..=5 {
        println!("全闭[1, 5] i: {}", i);
    }

    (1..5).for_each(|i| {
        println!("左闭右开[1, 5) i: {}", i);
    });

    // 由于 Rust 中的 char 是 Unicode 字符, 所以可以直接打印
    // 打印从 '你' 到 '我' 的所有字符
    // ('你'..='我').for_each(|x| print!("{}", x));
    // 统计从 '你' 到 '我' 的字符个数
    println!("'你' 到 '我' 之间的字符个数: {}", ('你'..='我').count());
}

// 使用 "r#keyword" 语法将关键字用作标识符
// 由于 "if" 和 "match" 是关键字, 所以不能直接用作函数名或参数名
// 这里, 我们分别使用 "r#if" 和 "r#match" 作为函数名和参数名
pub fn r#if(r#match: i32) {
    if r#match > 10 {
        println!("greater than 10");
        return;
    }

    match r#match {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("others"),
    }
}

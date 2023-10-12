pub fn basic_num() {
    // x type will be inferred by the compiler as i32, because we assigned it to 5
    let x = 5;

    // x = 10; // this will cause an error, because x is immutable by default

    // assign 10.1 to y, and the compiler will infer the type as f64
    let y = 10.1;

    // we can also explicitly define the type, here is an i64
    let z: i64 = 1234567890;

    // define a mutable variable
    let mut k = 10;
    println!("mutable variable ->k: {}", k);
    k = 20; // this is ok, because k is declared as mutable by "mut"

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
    let k_isize = 10isize;
    let k_usize = 10usize;

    let n0 = 1000_i64; // this is 1000 in i64
    let n1 = 1_0_0_0; // this is 1000 in i32
    let n2 = 1_000_00_00; // this is 10000000 in i32

    // call the add function
    let f = add(10, 20);

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
}

pub fn basic_others() {
    // here is a bool
    let is_active = true;


    // print the variable
    println!("is_active: {}", is_active);
}

fn add(x: i32, y: i32) -> i32 {
    // return x + y; // this is ok
    x + y // this is also ok, because the last expression will be returned
}

// use the "r#keyword" syntax to use the keyword as an identifier
pub fn r#if() {
    let r#match = 10;
    if r#match == 10 {
        println!("match");
    } else {
        println!("not match");
    }
}

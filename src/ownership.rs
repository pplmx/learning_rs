/*
栈(Stack): 先进后出, 占用已知固定大小的内存
堆(Heap): 对于大小未知或者大小可能变化的数据, 可以将数据存储在堆上
    Step 1: 申请一块足够大的内存
    Step 2: 将数据存储在这块内存上
    Step 3: 返回内存地址的指针
    Step 4: 将指针存储在栈上

所有权(Ownership): Rust 通过所有权系统管理堆上的数据, 避免内存泄漏和数据竞争
    1. Rust 中的每一个值都有一个被称为其所有者的变量
    2. 值在任一时刻只能被一个所有者拥有
    3. 当所有者(变量)离开作用域, 值将被销毁

所有权转移(Ownership Transfer): 将值从一个变量转移到另一个变量
    1. 通过赋值操作符(=)将值从一个变量转移到另一个变量
    2. 当值从一个变量转移到另一个变量时, 原变量将不再拥有该值
    3. 原变量将无法再访问该值

所有权借用(Ownership Borrowing): 通过引用(Reference)访问值, 而不是拥有值
    1. 通过 & 符号创建引用
    2. 引用不拥有值, 只是指向值的指针
    3. 引用不会影响值的生命周期
    4. 引用不会影响值的所有权
    5. 同一时刻, 要么有一个可变引用, 要么有多个不可变引用, 不能同时存在可变引用和不可变引用
*/

pub fn transferring() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // Error: value borrowed here after move
    println!("{}", s2);
}

// 所有权借用
pub fn borrowing() {
    let mut s = String::from("hello");

    // 不可变引用
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // NLL(Non-Lexical Lifetimes) 专门用于找到某个引用在作用域(})结束前就不再被使用的代码位置
    // r1, r2 在这之后不再被使用, 所以根据 NLL, r1, r2 的作用域在这里结束

    // 可变引用
    let r3 = &mut s;
    r3.push_str(", world");
    println!("{}", r3);
} // 新编译器中，r3作用域在这里结束

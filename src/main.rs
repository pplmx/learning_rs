use learning_rs::modules::rust_basics::{basic, ownership};

fn main() {
    basic::basic_num();
    println!("=====================");
    basic::basic_others();
    println!("=====================");
    basic::r#if(11);
    basic::r#if(8);
    basic::r#if(2);
    println!("=====================");

    ownership::transferring();
    ownership::borrowing();
}
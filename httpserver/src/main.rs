use std::env;

fn main() {
    //这个函数返回一个传递给程序的命令行参数的 迭代器（iterator）
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
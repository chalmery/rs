
use std::env;
use std::fs;

fn main() {
    //这个是能接受Unicode字符，如果是别的会抛异常
    let args:Vec<String> =  env::args().collect();
    //env::args_os()//返回OsString 这个能获取非Unicode字符
    // println!("{:?}",args);
    
    let query  = &args[1];
    let filename  = &args[2];

    println!("query: {:?}",query);
    println!("filename: {:?}",filename);

    //读取文件

    let contents = fs::read_to_string(filename)
    .expect("无法读取文件");

    println!("文件内容：\n {}",contents);

    
}

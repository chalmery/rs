
use std::env;
use std::fs;

fn main() {
    //这个是能接受Unicode字符，如果是别的会抛异常
    let args:Vec<String> =  env::args().collect();
    //env::args_os()//返回OsString 这个能获取非Unicode字符
    // println!("{:?}",args);

    let config = Config::new(&args);

    let contents = fs::read_to_string(config.filename)
    .expect("无法读取文件");

    println!("文件内容：\n {}",contents);

}
struct Config{
    query:String,
    filename:String,
}

impl Config {
    fn new(args:&[String]) -> Config{
        //牺牲内存换来少管理所有权
        let query  = args[1].clone();
        let filename  = args[2].clone(); 
        Config { query,filename }
    }
}



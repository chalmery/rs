
use std::env;
use std::fs;
use std::process;

fn main() {
    //这个是能接受Unicode字符，如果是别的会抛异常
    let args:Vec<String> =  env::args().collect();
    //env::args_os()//返回OsString 这个能获取非Unicode字符
    // println!("{:?}",args);

    let config = Config::new(&args)
    .unwrap_or_else(|err|{
        println!("解析参数错误：{}",err);
        process::exit(1);
    });

    let contents = fs::read_to_string(config.filename)
    .expect("无法读取文件");

    println!("文件内容：\n {}",contents);

}
struct Config{
    query:String,
    filename:String,
}

impl Config {
    //静态生命周期，其生命周期能够存活于整个程序期间。所有的字符串字面值都拥有 'static 生命周期
    fn new(args:&[String]) -> Result<Config,&'static str>{
        if args.len() <3{
            return  Err("没有输入内容");
        }
        //牺牲内存换来少管理所有权
        let query  = args[1].clone();
        let filename  = args[2].clone(); 
        Ok(Config { query,filename })
    }
}



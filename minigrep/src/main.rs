
use std::env;
use std::process;
use minigrep;

fn main() {
    //这个是能接受Unicode字符，如果是别的会抛异常
    let args:Vec<String> =  env::args().collect();
    //env::args_os()//返回OsString 这个能获取非Unicode字符
    // println!("{:?}",args);

    let config = minigrep::Config::new(&args)
    //unwrap表示打开包装，
    .unwrap_or_else(|err|{
        println!("解析参数错误：{}",err);
        process::exit(1);
    });
    //如果返回的是Result的Err类型也就是Box<dyn Error>>做处理
    if let Err(e) = minigrep::run(config){
         println!("应用错误：{}",e);
         process::exit(1);
    }

}

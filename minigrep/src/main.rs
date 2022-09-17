
use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    //这个是能接受Unicode字符，如果是别的会抛异常
    let args:Vec<String> =  env::args().collect();
    //env::args_os()//返回OsString 这个能获取非Unicode字符
    // println!("{:?}",args);

    let config = Config::new(&args)
    //unwrap表示打开包装，
    .unwrap_or_else(|err|{
        println!("解析参数错误：{}",err);
        process::exit(1);
    });
    //如果返回的是Result的Err类型也就是Box<dyn Error>>做处理
    if let Err(e) = run(config){
         println!("应用错误：{}",e);
         process::exit(1);
    }

}

//dyn 表示动态dynamic Box<dyn Error>相当于java的返回exception及其子类
fn run( config: Config) ->Result<(),Box<dyn Error>>{
    println!("要搜索的内容：{}",config.query);
    //?相当于kt的不处理错误，交给方法调用者
    let contents = fs::read_to_string(config.filename)?;
    println!("文件内容：\n {}",contents);
    Ok(())
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



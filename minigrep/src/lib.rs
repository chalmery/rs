use std::{error::Error, fs};




//dyn 表示动态dynamic Box<dyn Error>相当于java的返回exception及其子类
pub fn run( config: Config) ->Result<(),Box<dyn Error>>{
    println!("要搜索的内容：{}",config.query);
    //?相当于kt的不处理错误，交给方法调用者
    let contents = fs::read_to_string(config.filename)?;

    for result in search(&config.query, &contents) {
        println!("{}",result);
    }
    Ok(())
}


pub struct Config{
    pub query:String,
    pub filename:String,
}

impl Config {
    //静态生命周期，其生命周期能够存活于整个程序期间。所有的字符串字面值都拥有 'static 生命周期
    pub fn new(args:&[String]) -> Result<Config,&'static str>{
        if args.len() <3{
            return  Err("没有输入内容");
        }
        //牺牲内存换来少管理所有权
        let query  = args[1].clone();
        let filename  = args[2].clone(); 
        Ok(Config { query,filename })
    }
}


pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results 
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "github";
        let contents = "\
hello,
what,
rust is a lang,       
github is open.";
        assert_eq!(vec!["github is open."],
        search(query,contents))
    }
}


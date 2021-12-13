use std::env;
use std::fs;

struct Config{
    query: String,
    file_name: String,
}

impl Config{

    fn new(args: &Vec<String>) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("usage: cli [query] [file_name]");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();
        Ok(Config{
            query,
            file_name,
        })
    }

    fn search(&self) -> Result<Vec<String>, &'static str>{
        let mut result: Vec<String> = Vec::new();
        let contents = fs::read_to_string(&self.file_name).expect("fail to read file.");

        for line in contents.lines(){
            if line.contains(&self.query){
                result.push(line.to_string());
            }
        }
        return Ok(result);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Welcome to use search Program. Your args is {:?}", args);

    let config: Config = Config::new(&args).expect("Something wrong with use.");

    let result: Vec<String> = config.search().expect("Something wrong in search");

    println!("query: {}, file_name: {}", config.query, config.file_name);
    println!("result: ");

    for item in result{
        println!("-> {}", item);
    }
}

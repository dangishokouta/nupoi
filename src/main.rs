use clap::Parser;
use std::path::PathBuf;
use std::fs;

mod cli;

fn readfile(path_buf: PathBuf) -> String {
    return fs::read_to_string(path_buf).unwrap(); //unwrap値が入っていたら値を表示, NoneならPanic
}

fn returnline(text: &str) -> Vec<&str> {
    let line :Vec<&str> = text.split('\n').collect();
    return line;
}

fn return_count(count: i32) -> i32 {
    return count + 1;
}

fn main() {
    let _opts = cli::Options::parse();
    let path_buf = _opts.file;
    let csvtext = readfile(path_buf);
    let str2: &str = &csvtext;
    let lines :Vec<&str> = returnline(str2);
    let mut index = 0;
    for line in str2.lines() {
        // let line = line.unwrap(); //unwrapその2
        let mut c = 0;
        index = index + 1;

        for i in line.split(',') {
            if index !=1 {
                if i.len() == 0 {
                    c = return_count(c);
                }
            }
        }
        if c != 0 {
            println!("index: {}  null: {}", index, c);
            
        }
    }

}

fn hello(name: Option<String>) -> String {
    return format!("Hello, {}", if let Some(n) = name {
        n
    } else {
        "World".to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!("Hello, World", hello(None));
        assert_eq!("Hello, Dangisho", hello(Some("Dangisho".to_string())));
    }
    #[test]
    fn test_returnline(){
        let testreturn: &str = "1\n2";
        let a: &str = "1";
        let b: &str = "2";
        let teststr: &str = "c\nd\ne";
        let c: &str = "c";
        let d: &str = "d";
        let e: &str = "e";
        assert_eq!(vec![a,b], returnline(testreturn));
        assert_eq!(vec![c,d,e], returnline(teststr));
    }

    #[test]
    fn test_returncount(){
        assert_eq!(2, return_count(1));
    }
    #[test]
    fn test_readfile(){
        let path_buf = PathBuf::from("./test.csv");
        assert_eq!("\u{feff}name,item1,item2,item3,item4,item5\r\nTaro,3,4,1,5,\r\nJack,4,1,5,3,1\r\nAce,2,,5,3,3\r\nDan,4,4,3,4,3\r\nShin,,3,1,,4\r\nDaigo,3,,,,4",readfile(path_buf));
    }
}
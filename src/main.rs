#![allow(unused_imports)]

use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

struct Product{
    name: String,
    price: f32,
    in_stock: bool
}

enum Command{
    Insert(String),
    Update(i32, String),
    Delete(i32),
    Find
}

impl Command {
    fn serializable(&self) -> String{
        match self {
            Command::Insert(ref s) => format!("INSERT {}", s),
            Command::Update(ref i1, ref i2) => format!("UPDATE {} by id:{}", i2, i1),
            Command::Delete(ref i) => format!("DELETE by {}", i),
            Command::Find => "FIND".to_string(),
        }
    }
}

impl Product{
    fn new(name: String, price: f32, in_stock: bool) -> Product{
        Product{
            name,
            price,
            in_stock
        }
    }

    fn default_tax_rate() -> f32{
        0.1
    }

    fn calc_product_price_tax(&self, rate: f32) -> f32{
        if rate == 0.0{
            return  self.price * Product::default_tax_rate();
        }
        self.price * rate
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let mut buf = [0; 1024];
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_client(_stream)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    let cmd = Command::Insert(String::from("Hello, Rust!"));
    let json_str = cmd.serializable();
    println!("{}", json_str);

    if let Some(name) = get_db_user_name(2){
        print!("dbName is {}", name);
    }

    // let addr = TcpListener::local_addr(&listener);
    // println!("addr is {}", addr.unwrap().to_string())
}

fn add_string(p1: &mut String){
    p1.push_str(" is awsome");
}

fn print_string(p: &String){
    println!("{}",p)
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");

        if bytes_read == 0 {
            return;
        }

        stream.write_all(&buf[0..bytes_read]).expect("Failed to write to client");
    }
}

fn get_db_user_name(id: i32) -> Option<String>{
    let query = format!("select * from users where id = {}",id);
    if id == 1 {
        if let Ok(data) = query_db_data(&query){
            return Some(data)
        }else {
            println!("query db data error");
        };
        let res = query_db_data(&query);
        // res.ok();
        match res {
            Ok(data) => {
                return Some(data)
            }
            Err(e) => {
                println!("query db error: {}", e);
            }
        }
    };
    None
}

fn query_db_data(query: &String) ->Result<String, String>{
    let data_str = String::from("Fuck the world!");
    if query.is_empty() {
        Err(String::from("Query is empty"))
    }else {
        Ok(data_str)
    }
}
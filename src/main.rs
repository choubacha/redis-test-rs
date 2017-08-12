extern crate redis;
use redis::Commands;

fn main() {
    println!("creating client");
    let client = redis::Client::open("redis://redis:6379/0").unwrap();
    println!("grabbing connection");
    let conn = client.get_connection().unwrap();
    println!("setting a value");
    let _: () = conn.set("test", "value").unwrap();
    println!("getting the value");
    let set_value: String = conn.get("test").unwrap();
    println!("retrieved value: {}", set_value);
}
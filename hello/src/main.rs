
mod hello;
mod data_types;
mod constants;
mod operators;
mod scope;
mod stack;

fn main() {
    hello::run();
    println!("");
    
    data_types::run();
    println!("");
    
    constants::run();
    println!("");
    
    operators::run();
    println!("");
    
    scope::run();
    println!("");
    
    stack::run();

}
use actix::Actor;
use capabilities::Become;

use crate::functional_actix::{FunctionActor, FunctionCall};

mod capabilities;
mod functional_actix;

#[actix::main]
async fn main() {
    call_greet().await;
    cell_chest().await;
}

//Spritely Goblins tutorial examples

//Greeter:
async fn call_greet() {
    let gary_fn = FunctionActor::new(greeter(Become, "Gary".to_string())).start();
    let res = gary_fn.send(FunctionCall::new("Alice".to_string())).await;

    println!("{:?}", res);
}

fn greeter(_capability: Become, our_name: String) -> Box<dyn FnMut(String) -> String> {
    Box::new(move |your_name| format!("Hello {}, my name is {}", your_name, our_name))
}

//Cell greeter
async fn cell_chest() {
    let gary_fn = FunctionActor::new(cell_greeter(Become, "Gary".to_string())).start();
    let res = gary_fn.send(FunctionCall::new("Alice".to_string())).await;

    println!("cell {:?}", res);
}

fn cell_greeter(_capability: Become, our_name: String) -> Box<dyn FnMut(String) -> String> {
    Box::new(move |your_name| format!("Hello {}, my name is {}", your_name, our_name))
}

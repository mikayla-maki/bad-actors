use actix::Actor;
use capabilities::Become;

use crate::functional_actix::{FunctionActor, FunctionMsg};

mod capabilities;
mod functional_actix;

#[actix::main]
async fn main() {
    let gary_exec = FunctionActor::new(greeter(Become, "Gary".to_string())).start();
    let res = gary_exec
        .send(FunctionMsg::new(GreeterArgs {
            your_name: "Alice".to_string(),
        }))
        .await;

    println!("{:?}", res);
}

struct GreeterArgs {
    your_name: String,
}

fn greeter(_capability: Become, our_name: String) -> Box<dyn FnMut(GreeterArgs) -> String> {
    Box::new(move |args: GreeterArgs| format!("Hello {}, my name is {}", args.your_name, our_name))
}

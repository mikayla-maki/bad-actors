use std::marker::PhantomData;

use actix::dev::MessageResponse;
use actix::Message;
use actix::{Actor, Context, Handler};

//A message to call a function with the matching argument and return type
pub struct FunctionCall<A, R> {
    args: A,
    _marker: PhantomData<R>,
}

impl<A, R: 'static> Message for FunctionCall<A, R> {
    type Result = R;
}

impl<A, R> FunctionCall<A, R> {
    pub fn new(a: A) -> Self {
        Self {
            args: a,
            _marker: PhantomData,
        }
    }
}

///An actor that will store and execute a given closure
pub struct FunctionActor<A, R> {
    closure: Box<dyn FnMut(A) -> R>,
}

impl<A, R> FunctionActor<A, R> {
    pub fn new(f: Box<dyn FnMut(A) -> R>) -> Self {
        Self { closure: f }
    }
}

impl<A, R> Actor for FunctionActor<A, R>
where
    A: Unpin + 'static,
    R: Unpin + 'static,
{
    type Context = Context<Self>;
}

impl<A, R> Handler<FunctionCall<A, R>> for FunctionActor<A, R>
where
    A: Unpin + 'static,
    R: Unpin + MessageResponse<FunctionActor<A, R>, FunctionCall<A, R>> + 'static,
{
    type Result = R;

    fn handle(&mut self, msg: FunctionCall<A, R>, _ctx: &mut Self::Context) -> Self::Result {
        (self.closure)(msg.args)
    }
}

//Could also implement this as pure dynamic dispatch... but that makes me sad :(
// struct CallMethod1<A1, R1> {
//     msg: FunctionCall<A1, R1>,
// }

// struct CallMethod2<A2, R2> {
//     msg: FunctionCall<A2, R2>,
// }

// struct MethodActor<A1, R1, A2, R2> {
//     fn_1: FunctionActor<A1, R1>,
//     fn_2: FunctionActor<A2, R2>,
// }

// impl<A1, R1, A2, R2> Actor for MethodActor<A1, R1, A2, R2>
// where
//     A1: Unpin + 'static,
//     R1: Unpin + 'static,
//     A2: Unpin + 'static,
//     R2: Unpin + 'static,
// {
//     type Context = Context<Self>;
// }

// impl<A, R> Handler for MethodActor<>

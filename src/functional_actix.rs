use std::marker::PhantomData;

use actix::dev::MessageResponse;
use actix::Message;
use actix::{Actor, Context, Handler};

///A basic message
pub struct FunctionMsg<A, R> {
    args: A,
    _marker: PhantomData<R>,
}

impl<A, R: 'static> Message for FunctionMsg<A, R> {
    type Result = R;
}

impl<A, R> FunctionMsg<A, R> {
    pub fn new(a: A) -> Self {
        Self {
            args: a,
            _marker: PhantomData,
        }
    }
}

///An actor that can respond to this message
pub struct FunctionActor<A, R> {
    closure: Box<dyn FnMut(A) -> R>,
    _arg_marker: PhantomData<A>,
    _res_marker: PhantomData<R>,
}

impl<A, R> FunctionActor<A, R> {
    pub fn new(f: Box<dyn FnMut(A) -> R>) -> Self {
        Self {
            closure: f,
            _arg_marker: PhantomData,
            _res_marker: PhantomData,
        }
    }
}

impl<A: Unpin + 'static, R: Unpin + 'static> Actor for FunctionActor<A, R> {
    type Context = Context<Self>;
}

impl<A, R> Handler<FunctionMsg<A, R>> for FunctionActor<A, R>
where
    A: Unpin + 'static,
    R: Unpin + MessageResponse<FunctionActor<A, R>, FunctionMsg<A, R>> + 'static,
{
    type Result = R;

    fn handle(&mut self, msg: FunctionMsg<A, R>, _ctx: &mut Self::Context) -> Self::Result {
        (*self.closure)(msg.args)
    }
}

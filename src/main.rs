use std::{collections::HashMap, mem, thread};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Ref {
    id: usize,
}

impl Ref {
    fn null_ref() -> Ref {
        Ref { id: 0 }
    }

    fn _is_null(&self) -> bool {
        self.id == 0
    }
}

struct RefMaker {
    last_ref: Ref,
}

impl RefMaker {
    fn new() -> RefMaker {
        Self {
            last_ref: Ref { id: 0 },
        }
    }

    fn next_ref(&mut self) -> Ref {
        let next_ref = Ref {
            id: self.last_ref.id,
        };

        self.last_ref = next_ref;

        next_ref
    }
}

impl Default for RefMaker {
    fn default() -> Self {
        RefMaker::new()
    }
}

trait Obj {
    fn receive(&mut self, payload: Vec<u8>, ctx: &mut ExecutionCtx) -> bool;
}

struct Frame {}

struct Message {
    from: Ref,
    addr: Ref,
    payload: Vec<u8>,
}

impl Message {
    fn into_error(self, reason: &str) -> Message {
        let mut new_payload = reason.to_string().into_bytes();
        new_payload.extend(self.payload); //This is the worst error format error
        Self {
            from: self.addr,
            addr: self.from,
            payload: new_payload,
        }
    }
}

struct Vat {
    deliveries: Vec<Message>,
    received: Vec<Message>,
    heap: HashMap<Ref, Box<dyn Obj>>,
    _call_stack: Vec<Frame>,
    turn: u64,
}

struct ExecutionCtx {
    ref_maker: RefMaker,
    cur_obj: Ref,
    pending_deliveries: Vec<Message>,
    pending_objects: Vec<(Ref, Box<dyn Obj>)>,
    _turn: u64,
}

impl ExecutionCtx {
    fn send(&mut self, msg: Message) {
        self.pending_deliveries.push(msg);
    }

    fn create_obj(&mut self, obj: Box<dyn Obj>) -> Ref {
        let obj_ref = self.ref_maker.next_ref();
        self.pending_objects.push((obj_ref, obj));
        obj_ref
    }

    fn for_obj(&mut self, obj_addr: Ref) {
        self.cur_obj = obj_addr;
    }
}

fn make_ctx(vat: &Vat, ref_maker: RefMaker) -> ExecutionCtx {
    ExecutionCtx {
        ref_maker,
        cur_obj: Ref::null_ref(),
        pending_deliveries: Default::default(),
        pending_objects: Default::default(),
        _turn: vat.turn,
    }
}

impl Vat {
    fn new() -> Self {
        Self {
            deliveries: Default::default(),
            received: Default::default(),
            heap: Default::default(),
            _call_stack: Default::default(),
            turn: Default::default(),
        }
    }

    fn deliver_pending(&mut self) {
        mem::swap(&mut self.deliveries, &mut self.received);
        self.deliveries.clear()
    }

    //Start the vat. Note that this method does not return until the vat is empty
    fn start<F: FnOnce(&mut ExecutionCtx)>(mut self, start: F) -> () {
        let mut ref_maker = RefMaker::new();

        self.turn += 1;
        let mut ctx = make_ctx(&self, ref_maker);

        start(&mut ctx);

        ref_maker = mem::take(&mut ctx.ref_maker);

        for (obj_ref, obj) in ctx.pending_objects {
            self.heap.insert(obj_ref, obj);
        }

        self.deliveries.extend(ctx.pending_deliveries);

        loop {
            self.deliver_pending();
            self.turn += 1;

            let mut ctx = make_ctx(&self, ref_maker);

            let mut msgs = mem::take(&mut self.received);

            let mut should_delete = vec![];
            let mut should_fail = vec![];

            for msg in msgs.drain(..) {
                let obj_addr = msg.addr.clone();
                if let Some(mut obj) = self.heap.remove(&obj_addr) {
                    if !obj.receive(msg.payload, &mut ctx) {
                        ctx.for_obj(obj_addr);
                        should_delete.push(obj_addr);
                    }
                    self.heap.insert(obj_addr, obj);
                } else {
                    should_fail.push(msg)
                }
            }

            ref_maker = mem::take(&mut ctx.ref_maker);

            self.deliveries.extend(ctx.pending_deliveries);

            for msg in should_fail.into_iter() {
                self.deliveries
                    .push(msg.into_error("No such object exists"))
            }

            for addr in should_delete {
                self.heap.remove(&addr);
            }

            if self.heap.is_empty() {
                println!("Out of work, exiting...");
                break;
            }

            thread::yield_now()
        }
    }
}

fn main() {
    thread::spawn(|| {
        let vat = Vat::new();
        vat.start(|ctx| {
            let obj_ref = ctx.create_obj(Box::new(TestObj { counter: 100 }));
            ctx.send(Message {
                from: Ref::null_ref(),
                addr: obj_ref,
                payload: "Hello!".into(),
            })
        })
    })
    .join()
    .unwrap();
}

struct TestObj {
    counter: usize,
}

impl Obj for TestObj {
    fn receive(&mut self, payload: Vec<u8>, ctx: &mut ExecutionCtx) -> bool {
        println!(
            "On execution {} with payload {}",
            self.counter,
            std::str::from_utf8(&payload).unwrap()
        );

        self.counter -= 1;

        if self.counter > 0 {
            ctx.send(Message {
                from: ctx.cur_obj,
                addr: ctx.cur_obj,
                payload: "Hi".into(),
            });
            true
        } else {
            false
        }
    }
}

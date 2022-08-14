Do you want an actor framework that is:

- Single threaded
- Only allows communication inside that thread
- Is probably broken
- Flagarantly disregards Rust's type system
- Can both ping AND pong?????????

Then this is for you!

Just define your actor struct:


```rust
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
            false //Tell the runtime to drop us
        }
    }
}
```

Then create a new Vat and spawn your Actor:


```rust
fn main() {
    thread::spawn(|| {
        let vat = Vat::new();
        vat.start(|ctx| {
            let obj_ref = ctx.create_obj(Box::new(TestObj { counter: 10 }));

            ctx.send(Message {
                from: Ref::null_ref(),
                addr: obj_ref,
                payload: "Hello!".into(),
            });
        })
    })
    .join()
    .unwrap();
}
```

And your done! Fun for the whole family!! A great use of a Saturday night in my 20s!!!!!!!!

Here's ping pong:


```rust
fn main() {
    thread::spawn(|| {
        let vat = Vat::new();
        vat.start(|ctx| {
            ctx.create_obj(Box::new(PingObj {
                pong_addr: Ref::null_ref(),
            }));
        })
    })
    .join()
    .unwrap();
}

struct PingObj {
    pong_addr: Ref,
}

impl Obj for PingObj {
    fn on_start(&mut self, ctx: &mut ExecutionCtx) {
        self.pong_addr = ctx.create_obj(Box::new(PongObj {
            ping_addr: ctx.cur_obj,
        }));

        ctx.send(Message {
            from: ctx.cur_obj,
            addr: self.pong_addr,
            payload: vec![0],
        })
    }

    fn receive(&mut self, payload: Vec<u8>, ctx: &mut ExecutionCtx) -> bool {
        let new_counter = payload[0];

        println!("Ping! {}", new_counter);

        if new_counter <= 19u8 {
            ctx.send(Message {
                from: ctx.cur_obj,
                addr: self.pong_addr,
                payload: vec![new_counter + 1],
            });
            true
        } 
        if new_counter < 19u8 {
            true
        } else {
            false
        }
    }
}

struct PongObj {
    ping_addr: Ref,
}

impl Obj for PongObj {
    fn receive(&mut self, payload: Vec<u8>, ctx: &mut ExecutionCtx) -> bool {
        let new_counter = payload[0];

        println!("Pong! {}", new_counter);

        if new_counter < 20u8 {
            ctx.send(Message {
                from: ctx.cur_obj,
                addr: self.ping_addr,
                payload: vec![new_counter + 1],
            });
            true
        } else {
            false
        }
    }
}
```
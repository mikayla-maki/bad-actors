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
            false
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

There's also a ping-pong implementation using this framework, featuring the on_start method that Objs can overrride.
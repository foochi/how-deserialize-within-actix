use actix::*;
use actix_web::ws::{Client, Message, ProtocolError};
use futures::Future;

use serde::de;
use serde_json::from_str;

struct MyActor<T> {
    manager: EventManager<T>,
}

impl<T: 'static> Actor for MyActor<T> {
    type Context = Context<Self>;
}

impl<T: 'static> StreamHandler<Message, ProtocolError> for MyActor<T> {
    fn handle(&mut self, msg: Message, _ctx: &mut Context<Self>) {
        match msg {
            Message::Text(text) => {
                debug!("Received {}", text);

                for idx in 0..self.manager.events.len() {
                    let data =
                        from_str(&text).expect(&format!("Error when deserializing {:?}", text));
                    (self.manager.events[idx].handler)(data)
                }
            }
            _ => panic!(),
        }
    }
}

pub struct Event<T> {
    handler: Box<Fn(T) + 'static>,
}

pub struct EventManager<T> {
    events: Vec<Event<T>>,
}

impl<T: 'static> EventManager<T>
where
    T: serde::Deserialize<'static>,
{
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn capture<F>(&mut self, function: F)
    where
        F: for<'h> Fn(T) + 'static,
    {
        let event = Event {
            handler: Box::new(function),
        };
        self.events.push(event);
    }

    pub fn run(self) {
        let runner = System::new("example");

        debug!("run");

        Arbiter::spawn(
            Client::new("example")
                .connect()
                .map(|(reader, _writer)| {
                    MyActor::create(|ctx| {
                        MyActor::add_stream(reader, ctx);
                        MyActor { manager: self }
                    });
                })
                .map_err(|err| {}),
        );

        runner.run();
    }
}

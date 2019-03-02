use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use crate::protos::helloworld::{HelloRequest, HelloReply};
use crate::protos::helloworld_grpc::{self, Greeter};

#[derive(Clone)]
struct GreeterServer;

impl Greeter for GreeterServer {
    fn say_hello(&mut self, ctx: RpcContext, req: HelloRequest, sink: UnarySink<HelloReply>) {
        let mut hello_reply = HelloReply::new();
        hello_reply.set_message(String::from("Hello!"));

        let f = sink
            .success(hello_reply.clone())
            .map(move |_| println!("Responded with HelloReply"))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));

        ctx.spawn(f);
    }
}

pub fn serve() {
    let env = Arc::new(Environment::new(1));
    let service = helloworld_grpc::create_greeter(GreeterServer);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 49976)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}

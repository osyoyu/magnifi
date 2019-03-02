use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, ServerBuilder};

use crate::protos::magnifi_grpc;

use crate::grpc_handler::GrpcHandler;

pub fn serve(grpc_handler: GrpcHandler, host: String, port: u16) {
    let env = Arc::new(Environment::new(1));
    let service = magnifi_grpc::create_magnifi(grpc_handler);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind(host, port)
        .build()
        .unwrap();

    for &(ref host, port) in server.bind_addrs() {
        println!("Listening on {}:{}", host, port);
    }
    server.start();

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}

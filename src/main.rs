use clap::{Arg, App};

mod protos;
mod magnifi;
mod grpc_handler;
mod grpc_server;

fn main() {
    let matches =
        App::new("magnifi")
            .version("0.1.0")
            .arg(Arg::with_name("index_path")
                .short("i")
                .takes_value(true)
                .value_name("INDEX")
                .help("Path of index for benchmarking")
                .required(true))
            .arg(Arg::with_name("port")
                .short("p")
                .takes_value(true)
                .value_name("PORT")
                .help("Run magnifi on the specified port")
                .required(false))
            .arg(Arg::with_name("host")
                .short("b")
                .takes_value(true)
                .value_name("IP")
                .help("Binds magnifi to the specifed IP")
                .required(false))
            .get_matches();

    let host = matches.value_of("bind_addr").unwrap_or("127.0.0.1").to_string();
    let port = matches.value_of("port").unwrap_or("8192").parse::<u16>().unwrap();

    let magnifi = magnifi::MagnifiApp {};
    let grpc_handler = grpc_handler::GrpcHandler {
        magnifi: magnifi
    };
    grpc_server::serve(grpc_handler, host, port);
}

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use protobuf::RepeatedField;

// TODO: remove this
use std::path::PathBuf;
use tantivy_tokenizer_tiny_segmenter::tokenizer::TinySegmenterTokenizer;

use crate::protos::magnifi::{Document, Query, SearchReply};
use crate::protos::magnifi_grpc::{self, Magnifi};

use crate::searcher;

#[derive(Clone)]
struct MagnifiServer;

impl Magnifi for MagnifiServer {
    fn search(&mut self, ctx: RpcContext, req: Query, sink: UnarySink<SearchReply>) {
        // TODO: remove this
        let index_path = "../tantivy-jawp2";
        let index = tantivy::Index::open_in_dir(PathBuf::from(index_path)).expect(format!("Failed to open index in {}", index_path).as_str());
        index.tokenizers().register("tinyseg", TinySegmenterTokenizer {});
        let schema = index.schema();

        let query = req.get_body().to_string();
        let search_results = searcher::search(query);

        let mut docs_pb = RepeatedField::new();
        for result in search_results {
            let mut doc_pb = Document::new();
            doc_pb.set_doc_id(1);
            doc_pb.set_body(schema.to_json(&result.1).to_string());
            docs_pb.push(doc_pb);
        }


        let mut search_reply = SearchReply::new();
        search_reply.set_document(docs_pb);

        let f = sink
            .success(search_reply.clone())
            .map(move |_| println!("Responded with SearchReply"))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));

        ctx.spawn(f);
    }
}

pub fn serve(host: String, port: u16) {
    let env = Arc::new(Environment::new(1));
    let service = magnifi_grpc::create_magnifi(MagnifiServer);

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

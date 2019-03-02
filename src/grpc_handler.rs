use futures::Future;
use grpcio::{RpcContext, UnarySink};
use protobuf::RepeatedField;

use crate::protos::magnifi::{Document, Query, SearchReply};
use crate::protos::magnifi_grpc::Magnifi;

use crate::magnifi::MagnifiApp;

#[derive(Clone)]
pub struct GrpcHandler {
    // TODO: Change this to reference
    pub magnifi: MagnifiApp,
}

impl Magnifi for GrpcHandler {
    fn search(&mut self, ctx: RpcContext, req: Query, sink: UnarySink<SearchReply>) {
        let query = req.get_body().to_string();
        let search_results = (self.magnifi).search(query);

        // TODO: get JSON from MagnifiApp
        let schema = (self.magnifi).index().schema();

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

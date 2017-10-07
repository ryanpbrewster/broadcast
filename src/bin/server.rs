extern crate futures;
use futures::Stream;

extern crate grpc;

extern crate protobuf;
use protobuf::repeated::RepeatedField;

extern crate broadcast;

use std::thread;
use std::iter::FromIterator;
use std::sync::{Arc, RwLock};
use std::collections::HashSet;

use broadcast::proto::service_grpc::{Broadcast, BroadcastServer};
use broadcast::proto::service::{BroadcastRequest, BroadcastReply};

struct BroadcastImpl;

impl Broadcast for BroadcastImpl {
    fn broadcast(&self, _m: grpc::RequestOptions, req: BroadcastRequest) -> grpc::SingleResponse<BroadcastReply> {
        println!("broadcasting {}", req.msg);
        grpc::SingleResponse::completed(BroadcastReply::new())
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(50051);
    server.add_service(BroadcastServer::new_service_def(BroadcastImpl));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}
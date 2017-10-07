extern crate futures;
use futures::Stream;

extern crate grpc;

extern crate protobuf;
use protobuf::repeated::RepeatedField;

extern crate presence;

use std::thread;
use std::iter::FromIterator;
use std::sync::{Arc, RwLock};
use std::collections::HashSet;

use presence::proto::service_grpc::{Presence, PresenceServer};
use presence::proto::service::{Heartbeat, RegisterReply, ListRequest, ListReply};

struct PresenceImpl {
    active: Arc<RwLock<HashSet<String>>>,
}

impl Presence for PresenceImpl {
    fn register(&self, _m: grpc::RequestOptions, req: grpc::StreamingRequest<Heartbeat>) -> grpc::SingleResponse<RegisterReply> {
        let mut key: Option<String> = None;
        for hb in req.0.wait() {
            match hb {
                Ok(x) => {
                    if key.is_none() {
                        self.active.write().unwrap().insert(x.key.clone());
                        key = Some(x.key);
                    }
                    println!("registering heartbeat from {:?}", key);
                },
                Err(_) => {
                    println!("disconnected!");
                    for k in key.iter() {
                        self.active.write().unwrap().remove(k);
                    }
                    break
                }
            }
        }

        let mut r = RegisterReply::new();
        grpc::SingleResponse::completed(r)
    }

    fn list(&self, _m: grpc::RequestOptions, req: ListRequest) -> grpc::SingleResponse<ListReply> {
        let cur = self.active.read().unwrap();
        println!("listing current situation ({:?})", cur);

        let mut r = ListReply::new();
        r.set_history(RepeatedField::from_iter(cur.iter().cloned()));
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(50051);
    server.add_service(PresenceServer::new_service_def(PresenceImpl {
        active: Arc::new(RwLock::new(HashSet::new())),
    }));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}
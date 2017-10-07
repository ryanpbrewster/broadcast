extern crate futures;
use futures::Stream;

extern crate grpc;

extern crate broadcast;

use std::thread;

use broadcast::proto::service_grpc::{Broadcast, BroadcastServer};
use broadcast::proto::service::{BroadcastRequest, BroadcastReply, ListenRequest, ListenEvent};

struct BroadcastImpl;

impl Broadcast for BroadcastImpl {
    fn broadcast(
        &self,
        _opts: grpc::RequestOptions,
        req: BroadcastRequest,
    ) -> grpc::SingleResponse<BroadcastReply> {
        println!("broadcasting {}", req.msg);
        grpc::SingleResponse::completed(BroadcastReply::new())
    }

    fn listen(
        &self,
        _opts: grpc::RequestOptions,
        _req: ListenRequest,
    ) -> grpc::StreamingResponse<ListenEvent> {
        println!("listening...");

        let pongs = futures::stream::iter_ok(0..).map(|i: u32| {
            let mut pong = ListenEvent::new();
            pong.set_msg(format!("pong_{:06}", i));
            pong
        });
        grpc::StreamingResponse::no_metadata(pongs)
    }
}

const PORT: u16 = 50051;
fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(PORT);
    server.http.set_cpu_pool_threads(4);

    let broadcaster = BroadcastImpl;
    server.add_service(BroadcastServer::new_service_def(broadcaster));

    let _server = server.build().expect("server");
    println!("server listening at port {}", PORT);
    loop {
        thread::park();
    }
}

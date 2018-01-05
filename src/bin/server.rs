extern crate futures;
use futures::Stream;
use futures::sync::mpsc::UnboundedSender;

extern crate grpc;

extern crate broadcast;

use std::thread;
use std::sync::{Arc, RwLock};

use broadcast::proto::service_grpc::{Broadcast, BroadcastServer};
use broadcast::proto::service::{BroadcastRequest, BroadcastReply, ListenRequest, ListenEvent};

struct BroadcastImpl {
    receivers: Arc<RwLock<Vec<UnboundedSender<String>>>>,
}

impl Broadcast for BroadcastImpl {
    fn broadcast(
        &self,
        _opts: grpc::RequestOptions,
        req: BroadcastRequest,
    ) -> grpc::SingleResponse<BroadcastReply> {
        println!("broadcasting {}", req.msg);
        {
            let mut listeners = self.receivers.write().expect(
                "acquiring write-lock for receivers",
            );
            let init_count = listeners.len();
            listeners.retain(|tx| {
                tx.unbounded_send(req.msg.clone()).is_ok()
            });
            println!("pushed to {} receivers, {} ok", init_count, listeners.len());
        }
        grpc::SingleResponse::completed(BroadcastReply::new())
    }

    fn listen(
        &self,
        _opts: grpc::RequestOptions,
        _req: ListenRequest,
    ) -> grpc::StreamingResponse<ListenEvent> {
        println!("listening...");

        let (tx, rx) = futures::sync::mpsc::unbounded::<String>();
        {
            let mut receivers = self.receivers.write().expect("acquiring write-lock for receivers");
            println!("{} -> {} receivers", receivers.len(), receivers.len() + 1);
            receivers.push(tx);
        }
        grpc::StreamingResponse::no_metadata(rx.map(|msg: String| {
            let mut pong = ListenEvent::new();
            pong.set_msg(msg);
            pong
        }).map_err(|()| {
            grpc::Error::Other("TODO(rpb): add better error message")
        }))
    }
}

const PORT: u16 = 50051;
fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(PORT);
    server.http.set_cpu_pool_threads(4);

    let broadcaster = BroadcastImpl { receivers: Arc::new(RwLock::new(Vec::new())) };
    server.add_service(BroadcastServer::new_service_def(broadcaster));

    let _server = server.build().expect("server");
    println!("server listening at port {}", PORT);
    loop {
        thread::park();
    }
}

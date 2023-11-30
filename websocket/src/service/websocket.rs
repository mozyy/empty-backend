use std::net::SocketAddr;

use empty_utils::diesel::db;
use empty_utils::errors::{ErrorConvert, Result};

use tokio::net::TcpStream;

pub struct Service {
    db: db::DbPool,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            db: db::DbPool::new("websocket_v2"),
        }
    }
}

impl Service {
    async fn handle_connection(&self, raw_stream: TcpStream, addr: SocketAddr) -> Result {
        log::debug!("Incoming TCP connection from: {}", addr);
        let _ws_stream = tokio_tungstenite::accept_async(raw_stream)
            .await
            .ok_or_invalid()?;
        log::debug!("WebSocket connection established: {}", addr);
        todo!()
    }
}

// async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
//   println!("Incoming TCP connection from: {}", addr);

//   let ws_stream = tokio_tungstenite::accept_async(raw_stream)
//       .await
//       .expect("Error during the websocket handshake occurred");
//   println!("WebSocket connection established: {}", addr);

//   // Insert the write part of this peer to the peer map.
//   let (tx, rx) = unbounded();
//   peer_map.lock().unwrap().insert(addr, tx);

//   let (outgoing, incoming) = ws_stream.split();

//   let broadcast_incoming = incoming.try_for_each(|msg| {
//       println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
//       let peers = peer_map.lock().unwrap();

//       // We want to broadcast the message to everyone except ourselves.
//       let broadcast_recipients =
//           peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

//       for recp in broadcast_recipients {
//           recp.unbounded_send(msg.clone()).unwrap();
//       }

//       future::ok(())
//   });

//   let receive_from_others = rx.map(Ok).forward(outgoing);

//   pin_mut!(broadcast_incoming, receive_from_others);
//   future::select(broadcast_incoming, receive_from_others).await;

//   println!("{} disconnected", &addr);
//   peer_map.lock().unwrap().remove(&addr);
// }

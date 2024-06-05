// socketcan/examples/tokio_print_frames.rs
//
// Example application for using Tokio with socketcan-rs.
//
// This file is part of the Rust 'socketcan-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

//! A SocketCAN example using Tokio.
//!
//! This receives CAN frames and prints them to the console.
//!

use futures_util::StreamExt;
use socketcan::{tokio::CanSocket, CanFrame, SocketOptions, Timestamp};
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let iface = env::args().nth(1).unwrap_or_else(|| "vcan0".into());
    let mut sock = CanSocket::open(&iface).unwrap();

    sock.set_timestamps(true)?;

    println!("Reading on {}", iface);

    while let Some(res) = sock.next().await {
        match res {
            Ok((CanFrame::Data(frame), ts)) => print_frame(ts, &frame),
            Ok((CanFrame::Remote(frame), ts)) => print_frame(ts, &frame),
            Ok((CanFrame::Error(frame), ts)) => print_frame(ts, &frame),
            Err(err) => eprintln!("{}", err),
        }
    }

    Ok(())
}

fn print_frame<T: std::fmt::Debug>(ts: Option<Timestamp>, frame: &T) {
    if let Some(ts) = ts {
        println!("[{ts}] {frame:?}")
    } else {
        println!("{frame:?}")
    }
}

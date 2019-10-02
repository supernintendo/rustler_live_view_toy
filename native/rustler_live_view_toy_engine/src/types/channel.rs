use bytes::Bytes;
use futures::sync::mpsc;

/// Packet receiver
pub type Rx = mpsc::UnboundedReceiver<Bytes>;

/// Packet transmitter
pub type Tx = mpsc::UnboundedSender<Bytes>;

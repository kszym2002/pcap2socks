//! Support for serializing and deserializing layers.

use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
use std::io;

pub mod arp;
pub mod ethernet;
pub mod ipv4;
pub mod tcp;
pub mod udp;

/// Represents the kind of the layer.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LayerKind(u8);

impl Display for LayerKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                LayerKinds::Ethernet => "Ethernet",
                LayerKinds::Arp => "ARP",
                LayerKinds::Ipv4 => "IPv4",
                LayerKinds::Tcp => "TCP",
                LayerKinds::Udp => "UDP",
                _ => "unknown",
            }
        )
    }
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
/// Represents all the layer kinds.
pub mod LayerKinds {
    use super::LayerKind;

    /// Represents the layer kind of Ethernet.
    pub const Ethernet: LayerKind = LayerKind(0);
    /// Represents the layer kind of ARP.
    pub const Arp: LayerKind = LayerKind(1);
    /// Represents the layer kind of IPv4.
    pub const Ipv4: LayerKind = LayerKind(2);
    /// Represents the layer kind of TCP.
    pub const Tcp: LayerKind = LayerKind(3);
    /// Represents the layer kind of UDP.
    pub const Udp: LayerKind = LayerKind(4);
}

/// Represents a layer.
pub trait Layer: Display {
    // Get the kind of the `Layer`.
    fn kind(&self) -> LayerKind;

    // Get The length of the `Layer` when converted into a byte-array.
    fn len(&self) -> usize;

    // Serialize the `Layer` into a byte-array.
    fn serialize(&self, buffer: &mut [u8], n: usize) -> io::Result<usize>;

    // Serialize the `Layer` into a byte-array with payload.
    fn serialize_with_payload(
        &self,
        buffer: &mut [u8],
        payload: &[u8],
        n: usize,
    ) -> io::Result<usize>;
}

#[derive(Clone, Debug)]
/// Enumeration of layers.
pub enum Layers {
    /// Represents the Ethernet layer.
    Ethernet(ethernet::Ethernet),
    /// Represents the ARP layer.
    Arp(arp::Arp),
    /// Represents the IPv4 layer.
    Ipv4(ipv4::Ipv4),
    /// Represents the TCP layer.
    Tcp(tcp::Tcp),
    /// Represents the UDP layer.
    Udp(udp::Udp),
}

impl Display for Layers {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Layers::Ethernet(ref layer) => layer.fmt(f),
            Layers::Arp(ref layer) => layer.fmt(f),
            Layers::Ipv4(ref layer) => layer.fmt(f),
            Layers::Tcp(ref layer) => layer.fmt(f),
            Layers::Udp(ref layer) => layer.fmt(f),
        }
    }
}

impl Layer for Layers {
    fn kind(&self) -> LayerKind {
        match self {
            Layers::Ethernet(ref layer) => layer.kind(),
            Layers::Arp(ref layer) => layer.kind(),
            Layers::Ipv4(ref layer) => layer.kind(),
            Layers::Tcp(ref layer) => layer.kind(),
            Layers::Udp(ref layer) => layer.kind(),
        }
    }

    fn len(&self) -> usize {
        match self {
            Layers::Ethernet(ref layer) => layer.len(),
            Layers::Arp(ref layer) => layer.len(),
            Layers::Ipv4(ref layer) => layer.len(),
            Layers::Tcp(ref layer) => layer.len(),
            Layers::Udp(ref layer) => layer.len(),
        }
    }

    fn serialize(&self, buffer: &mut [u8], n: usize) -> io::Result<usize> {
        match self {
            Layers::Ethernet(ref layer) => layer.serialize(buffer, n),
            Layers::Arp(ref layer) => layer.serialize(buffer, n),
            Layers::Ipv4(ref layer) => layer.serialize(buffer, n),
            Layers::Tcp(ref layer) => layer.serialize(buffer, n),
            Layers::Udp(ref layer) => layer.serialize(buffer, n),
        }
    }

    fn serialize_with_payload(
        &self,
        buffer: &mut [u8],
        payload: &[u8],
        n: usize,
    ) -> io::Result<usize> {
        match self {
            Layers::Ethernet(ref layer) => layer.serialize_with_payload(buffer, payload, n),
            Layers::Arp(ref layer) => layer.serialize_with_payload(buffer, payload, n),
            Layers::Ipv4(ref layer) => layer.serialize_with_payload(buffer, payload, n),
            Layers::Tcp(ref layer) => layer.serialize_with_payload(buffer, payload, n),
            Layers::Udp(ref layer) => layer.serialize_with_payload(buffer, payload, n),
        }
    }
}

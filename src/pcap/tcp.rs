use pnet::packet::ipv4::Ipv4;
use pnet::packet::tcp::{self, MutableTcpPacket, Tcp, TcpOptionPacket, TcpPacket};

/// Creates an `Tcp` according to the given TCP packet.
pub fn parse_tcp(packet: &TcpPacket) -> Tcp {
    Tcp {
        source: packet.get_source(),
        destination: packet.get_destination(),
        sequence: packet.get_sequence(),
        acknowledgement: packet.get_acknowledgement(),
        data_offset: packet.get_data_offset(),
        reserved: packet.get_reserved(),
        flags: packet.get_flags(),
        window: packet.get_window(),
        checksum: packet.get_checksum(),
        urgent_ptr: packet.get_urgent_ptr(),
        options: packet.get_options(),
        payload: vec![],
    }
}

/// Serializes an `Tcp` with `Ipv4`.
pub fn serialize_ipv4_tcp(
    layer: &Tcp,
    ipv4_layer: &Ipv4,
    n: usize,
    buffer: &mut [u8],
) -> Result<usize, String> {
    let mut packet = match MutableTcpPacket::new(buffer) {
        Some(packet) => packet,
        None => return Err(format!("connot serialize TCP layer")),
    };

    packet.populate(layer);
    let mut data_offset = 20;
    for option in layer.options.iter() {
        data_offset = data_offset + TcpOptionPacket::packet_size(option);
    }
    packet.set_data_offset((data_offset / 4) as u8);

    // Checksum
    let checksum = tcp::ipv4_checksum(
        &packet.to_immutable(),
        &ipv4_layer.source,
        &ipv4_layer.destination,
    );
    packet.set_checksum(checksum);

    Ok(TcpPacket::packet_size(layer) + n)
}
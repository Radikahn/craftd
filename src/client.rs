struct QueryPacket {
    packet_type: i8,
    session_id: i8,
}

// Packet infra is provided via [Query Minecraft Wiki](https://minecraft.wiki/w/Query)
// Some code is translated from [McClient](https://github.com/Sch8ill/mcclient-lib/blob/master)

fn pack(query: QueryPacket, payload: &[u8]) -> &[u8] {
    let mut packet: &[u8] = "254223".as_bytes();
    packet += &query.packet_type.to_ne_bytes();
    packet += &query.session_id.to_ne_bytes();
    packet += payload;
    return packet;
}

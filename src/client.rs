struct QueryPacket {
    packet_type: u8,
    session_id: i32,
}

// Packet infra is provided via [Query Minecraft Wiki](https://minecraft.wiki/w/Query)
// Some code is translated from [McClient](https://github.com/Sch8ill/mcclient-lib/blob/master)

const MAGIC: [u8; 2] = [0xFE, 0xFD];

fn pack(query: QueryPacket, payload: &[u8]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(2 + 1 + 4 + payload.len());
    packet.extend_from_slice(&MAGIC);
    packet.push(query.packet_type);
    packet.extend_from_slice(&(query.session_id & 0x0F0F_0F0F).to_be_bytes());
    return packet;
}

pub enum TransportProtocol {
    Tcp,
    Udp,
}

impl TransportProtocol {
    pub fn from(val: &str) -> TransportProtocol {
        match val {
            "TCP" => TransportProtocol::Tcp,
            "UDP" => TransportProtocol::Udp,
            _ => TransportProtocol::Udp,
        }
    }
}

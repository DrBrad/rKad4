pub enum AddressType {

    IPv4,
    IPv6
}

impl AddressType {
    fn address_length(&self) -> u32 {
        match self {
            AddressType::IPv4 => 4,
            AddressType::IPv6 => 16,
        }
    }
}

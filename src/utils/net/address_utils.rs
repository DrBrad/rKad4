use std::net::{IpAddr, SocketAddr};

const LOCAL_BROADCAST: [u8; 4] = [0xff, 0xff, 0xff, 0xff];

/*
private static final NetMask V4_MAPPED;

static {
    try{
        // ::ffff:0:0/96
        V4_MAPPED = new NetMask(Inet6Address.getByAddress(null, new byte[]{
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                (byte) 0xff,
                (byte) 0xff,
                0x00,
                0x00,
                0x00,
                0x00
        }, null), 96);
    }catch(UnknownHostException e){
        throw new Error("Unable to set Global Unicast IPv4 static variable.");
    }
}
*/

pub fn is_bogon(addr: SocketAddr) -> bool {
    !(addr.port() > 0 && is_global_unicast(addr.ip()))
}

pub fn is_teredo(addr: IpAddr) -> bool {
    if addr.is_ipv6() {
        /*
        byte[] buf = address.getAddress();
        return buf[0] == 0x20 &&
            buf[1] == 0x01 &&
            buf[2] == 0x00 &&
            buf[3] == 0x00;
        */
    }

    false
}

pub fn is_global_unicast(addr: IpAddr) -> bool {
    match addr {
        IpAddr::V4(v4) => {
            if v4.octets()[0] == 0 || v4.octets() == LOCAL_BROADCAST {
                return false;
            }
        }
        IpAddr::V6(v6) => {
            if (v6.segments()[0] & 0xfe) == 0xfc /*|| v4_mapped_contains(address) || v6.is_ipv4_compatible()*/ {
                //(V4_MAPPED.contains(address)
                // || ((Inet6Address) address).isIPv4CompatibleAddress())
                return false;
            }
        }
    }

    match addr {
        IpAddr::V4(v4) => {
            !(v4.is_unspecified() || v4.is_loopback() || v4.is_link_local() || v4.is_multicast() || v4.is_broadcast())
        }
        IpAddr::V6(v6) => {
            !(v6.is_unspecified() || v6.is_loopback()/* || v6.is_unicast_link_local()*/ || v6.is_multicast()/* || v6.is_unicast_site_local()*/)
        }
    }
}

pub fn pack_address() -> Vec<u8> {
    let mut buf = vec![0; 2];

    buf
}

/*
pub fn unpack_addr() -> SocketAddr {

}
*/

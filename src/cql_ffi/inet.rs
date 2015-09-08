//use cql_ffi::types::cass_uint8_t;
use cql_bindgen::CassInet as _CassInet;
use cql_bindgen::cass_inet_init_v4;
use cql_bindgen::cass_inet_init_v6;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::default::Default;

#[repr(C)]
pub struct CassInet(pub _CassInet);

impl Default for CassInet {
    fn default() -> CassInet {
        unsafe {
            ::std::mem::zeroed()
        }
    }
}

pub trait AsCassInet {
    fn as_cass_inet(&self) -> CassInet;
}

impl AsCassInet for SocketAddr {
    fn as_cass_inet(&self) -> CassInet {
        match *self {
            SocketAddr::V4(ipv4_addr) => {
                unsafe {
                    CassInet(cass_inet_init_v4(ipv4_addr.ip().octets().as_ptr()))
                }
            }
            SocketAddr::V6(ipv6_addr) => {
                unsafe {
                    let seg = ipv6_addr.ip().segments();
                //FIXME does this really work?
                    CassInet(cass_inet_init_v6(seg.as_ptr() as *const u8))
                }
            }
        }
        //~ let foo:_CassInet = Default::default();
        //~ CassInet(foo)
    }
}

pub trait FromCassInet {
    fn from_cass_inet(inet: CassInet) -> Self;
}

impl FromCassInet for Ipv4Addr {
    fn from_cass_inet(inet: CassInet) -> Self {
        let raw_addr: [u8; 16] = inet.0.address;
        match inet.0.address_length {
            4 => Ipv4Addr::new(raw_addr[0], raw_addr[1], raw_addr[2], raw_addr[3]),
            16 => panic!(),
            unsupported => panic!("impossible inet type: {:?}", unsupported),
        }
    }
}

impl FromCassInet for Ipv6Addr {
    fn from_cass_inet(inet: CassInet) -> Self {
        let raw_addr: [u8; 16] = inet.0.address;
        match inet.0.address_length {
            4 => panic!(),
            16 => Ipv6Addr::new((raw_addr[1]  as u16) << (8 + raw_addr[0]  as u16),
                                (raw_addr[3]  as u16) << (8 + raw_addr[2]  as u16),
                                (raw_addr[5]  as u16) << (8 + raw_addr[4]  as u16),
                                (raw_addr[7]  as u16) << (8 + raw_addr[6]  as u16),
                                (raw_addr[9]  as u16) << (8 + raw_addr[8]  as u16),
                                (raw_addr[11] as u16) << (8 + raw_addr[10] as u16),
                                (raw_addr[13] as u16) << (8 + raw_addr[12] as u16),
                                (raw_addr[15] as u16) << (8 + raw_addr[14] as u16)),
            unsupported => panic!("impossible inet type: {}", unsupported),
        }
    }
}

impl CassInet {
    pub fn cass_inet_init_v4(address: *const u8) -> CassInet {
        unsafe {
            CassInet(cass_inet_init_v4(address))
        }
    }

    pub fn cass_inet_init_v6(address: *const u8) -> CassInet {
        unsafe {
            CassInet(cass_inet_init_v6(address))
        }
    }
}

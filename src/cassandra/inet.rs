use std::str::FromStr;
use std::string::ToString;
use std::mem;
use std::ffi::CString;
// use std::ffi::NulError;
use std::ffi::CStr;
use std::net::{Ipv4Addr, Ipv6Addr};
use cassandra_sys::CassInet as _Inet;
use cassandra_sys::cass_inet_init_v4;
use cassandra_sys::cass_inet_init_v6;
use cassandra_sys::cass_inet_string;
use cassandra_sys::cass_inet_from_string;
use cassandra_sys::CASS_OK;
use std::net::SocketAddr;
use std::default::Default;
use std::fmt::{Debug, Formatter};
use std::fmt;
use cassandra::util::Protected;

use cassandra::error::CassError;
// use cassandra::error::CassLibError;

#[repr(C)]
///Cassandra's version of an IP address

pub struct Inet(_Inet);

impl Debug for Inet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unimplemented!();
    }
}

impl Protected<_Inet> for Inet {
    fn inner(&self) -> _Inet {
        self.0
    }
    fn build(inner: _Inet) -> Self {
        Inet(inner)
    }
}

impl Default for Inet {
    fn default() -> Inet {
        unsafe { ::std::mem::zeroed() }
    }
}

///Lets various things get converted to a Inet
pub trait AsInet {
    ///Converts to a Cassandra Inet
    fn as_cass_inet(&self) -> Inet;
}

impl AsInet for SocketAddr {
    fn as_cass_inet(&self) -> Inet {
        match *self {
            SocketAddr::V4(ipv4_addr) => unsafe { Inet(cass_inet_init_v4(ipv4_addr.ip().octets().as_ptr())) },
            SocketAddr::V6(ipv6_addr) => {
                unsafe {
                    let seg = ipv6_addr.ip().segments();
                    // FIXME does this really work?
                    Inet(cass_inet_init_v6(seg.as_ptr() as *const u8))
                }
            }
        }
        // ~ let foo:_Inet = Default::default();
        // ~ Inet(foo)
    }
}

///The types of errors that can occur when trying to parse an Inet String
// pub enum InetParseError {
//    ///Don't put a null in a string, silly!
//    NulInString(NulError),
//    ///Not a valiid address
//    LibBadParams(CassLibError),
// }

impl FromStr for Inet {
    type Err = CassError;

    fn from_str(s: &str) -> Result<Self, CassError> {
        unsafe {
            let mut inet = mem::zeroed();

            let s = CString::new(s).unwrap();
            match cass_inet_from_string(s.as_ptr(), &mut inet) {
                CASS_OK => Ok(Inet(inet)),
                err => Err(CassError::build(err)),
            }
        }
    }
}

impl ToString for Inet {
    fn to_string(&self) -> String {
        unsafe {
            let mut inet_str = mem::zeroed();
            cass_inet_string(self.0, &mut inet_str);
            CStr::from_ptr(&inet_str).to_string_lossy().into_owned()
        }
    }
}

///Converts from an Cassandra Inet address
pub trait FromInet {
    ///Converts from an Cassandra Inet address
    fn from_cass_inet(inet: Inet) -> Self;
}

impl FromInet for Ipv4Addr {
    fn from_cass_inet(inet: Inet) -> Self {
        let raw_addr: [u8; 16] = inet.0.address;
        match inet.0.address_length {
            4 => Ipv4Addr::new(raw_addr[0], raw_addr[1], raw_addr[2], raw_addr[3]),
            16 => panic!(),
            unsupported => panic!("impossible inet type: {:?}", unsupported),
        }
    }
}

impl FromInet for Ipv6Addr {
    fn from_cass_inet(inet: Inet) -> Self {
        let raw_addr: [u8; 16] = inet.0.address;
        match inet.0.address_length {
            4 => panic!(),
            16 => {
                Ipv6Addr::new((raw_addr[1] as u16) << (8 + raw_addr[0] as u16),
                              (raw_addr[3] as u16) << (8 + raw_addr[2] as u16),
                              (raw_addr[5] as u16) << (8 + raw_addr[4] as u16),
                              (raw_addr[7] as u16) << (8 + raw_addr[6] as u16),
                              (raw_addr[9] as u16) << (8 + raw_addr[8] as u16),
                              (raw_addr[11] as u16) << (8 + raw_addr[10] as u16),
                              (raw_addr[13] as u16) << (8 + raw_addr[12] as u16),
                              (raw_addr[15] as u16) << (8 + raw_addr[14] as u16))
            }
            unsupported => panic!("impossible inet type: {}", unsupported),
        }
    }
}

impl Inet {
    ///Constructs an inet v4 object.
    pub fn cass_inet_init_v4(address: Ipv4Addr) -> Inet {
        unsafe { Inet(cass_inet_init_v4(address.octets().as_ptr())) }
    }

    ///Constructs an inet v6 object.
    pub fn cass_inet_init_v6(address: Ipv6Addr) -> Inet {
        unsafe { Inet(cass_inet_init_v6(address.segments().as_ptr() as *const u8)) }
    }
}

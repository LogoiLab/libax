use std::net::Ipv4Addr;

trait ToIpv4 {
    fn to_ipv4(self) -> Ipv4Addr;
}

impl ToIpv4 for u32 {
    fn to_ipv4(self) -> Ipv4Addr {
        return Ipv4Addr::from(self);
    }
}

trait Tou32 {
    fn to_u32(self) -> u32;
}

impl Tou32 for Ipv4Addr {
    fn to_u32(self) -> u32 {
        return self.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_to_ipv4() {
        let ip: Ipv4Addr = 123456789.to_ipv4();
        assert_eq!(ip, Ipv4Addr::new(7, 91, 205, 21));
    }
    #[test]
    fn ipv4_to_u32() {
        let num: u32 = Ipv4Addr::new(7, 91, 205, 21).to_u32();
        assert_eq!(num, 123456789);
    }
}

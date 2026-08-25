mod ipv4 {
    pub struct Ipv4;

    impl Ipv4 {
        /// Returns true if the argument is valid, otherwise false.
        pub fn is_ipv4(ipv4_address: &str) -> bool {
            Ipv4::parse_string_ipv4(ipv4_address).is_ok()
        }

        pub fn parse_string_ipv4(ipv4_address: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>>  {
            let octets = ipv4_address
                .split('.')
                .map(|octet| octet.parse::<u8>())
                .collect::<Result<Vec<_>, _>>()?;

            if octets.len() != 4 {
                return Err("IPv4 address must contain exactly 4 octets".into());
            }

            Ok(octets)
        }
    }
}

pub use ipv4::Ipv4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_ipv4_rejects_empty_string() {
        assert!(!Ipv4::is_ipv4(""));
    }

    #[test]
    fn is_ipv4_rejects_octet_greater_than_255() {
        let result = Ipv4::is_ipv4("192.168.1.256");

        assert!(!result);
    }

    #[test]
    fn is_ipv4_rejects_non_numeric_octet() {
        let result = Ipv4::is_ipv4("192.168.hello.10");

        assert!(!result);
    }

    #[test]
    fn is_ipv4_rejects_too_few_octets() {
        assert!(!Ipv4::is_ipv4("192.168.1"));
    }

    #[test]
    fn is_ipv4_rejects_too_many_octets() {
        assert!(!Ipv4::is_ipv4("192.168.1.10.20"));
    }    
}

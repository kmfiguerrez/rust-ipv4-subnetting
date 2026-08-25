mod ipv4 {
    pub struct Ipv4;

    impl Ipv4 {
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
    fn parse_string_ipv4_valid_address() {
        let result = Ipv4::parse_string_ipv4("192.168.1.10");

        assert_eq!(result.unwrap(), vec![192, 168, 1, 10]);
    }
}

mod ipv4 {
    pub struct Ipv4;

    impl Ipv4 {
        /// Returns true if the argument is valid, otherwise false.
        pub fn is_ipv4(ipv4_address: &str) -> bool {
            Self::parse_string_ipv4(ipv4_address).is_ok()
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

        /// Coverts decimal into binary using 8-bit representaion.
        pub fn decimal_to_binary(decimal: u8) -> String {
            format!("{decimal:08b}")
        }

        pub fn binary_to_decimal(binary: &str) -> Result<u8, &'static str> {
            if binary.len() != 8 {
                return Err("binary must contain exactly 8 bits");
            }

            let mut decimal = 0;

            for bit in binary.chars() {
                decimal *= 2;

                match bit {
                    '0' => {}
                    '1' => decimal += 1,
                    _ => return Err("binary can only contain 0 or 1"),
                }
            }

            Ok(decimal)
        }        
    }
}





pub use ipv4::Ipv4;

#[cfg(test)]
mod tests {
    use super::*;

    mod is_ipv4 {
        use super::*;

        #[test]
        fn it_rejects_empty_string() {
            assert!(!Ipv4::is_ipv4(""));
        }

        #[test]
        fn it_rejects_octet_greater_than_255() {
            let result = Ipv4::is_ipv4("192.168.1.256");

            assert!(!result);
        }

        #[test]
        fn it_rejects_negative_octet() {
            let result = Ipv4::is_ipv4("192.168.1.-1");

            assert!(!result);
        }    

        #[test]
        fn it_rejects_non_numeric_octet() {
            let result = Ipv4::is_ipv4("192.168.hello.10");

            assert!(!result);
        }

        #[test]
        fn it_rejects_too_few_octets() {
            assert!(!Ipv4::is_ipv4("192.168.1"));
        }

        #[test]
        fn it_rejects_too_many_octets() {
            assert!(!Ipv4::is_ipv4("192.168.1.10.20"));
        }
    }

    mod parse_string_ipv4 {
        use super::*;

        #[test]
        fn it_should_return_vec_of_u8_on_valid_input() {
            assert_eq!(Ipv4::parse_string_ipv4("192.168.10.5").unwrap(), vec![192,168,10,5]);
        }    
    }

    mod decimal_to_binary {
        use super::*;

        #[test]
        fn it_always_returns_eight_bits() {
            // let result = Ipv4::decimal_to_binary(8);
            // println!("Result: {result}");
            for decimal in 0..=255 {
               assert_eq!(Ipv4::decimal_to_binary(decimal).len(), 8);
            }
        }

        #[test]
        fn converts_zero() {
            assert_eq!(Ipv4::decimal_to_binary(0), "00000000");
        }

        #[test]
        fn converts_maximum_u8() {
            let result = Ipv4::decimal_to_binary(255);
            // println!("Result: {result}");
            assert_eq!(result, "11111111");
        }

        #[test]
        fn it_converts_values_with_leading_zeroes() {
            assert_eq!(Ipv4::decimal_to_binary(13), "00001101");
            assert_eq!(Ipv4::decimal_to_binary(42), "00101010");
        }        
    }

    mod binary_to_decimal {
        use super::*;

        #[test]
        fn it_rejects_binary_with_wrong_length() {
            assert!(Ipv4::binary_to_decimal("101").is_err());
            assert!(Ipv4::binary_to_decimal("101011110101").is_err());
            assert!(Ipv4::binary_to_decimal("").is_err());
        }

        #[test]
        fn it_rejects_non_binary_characters() {
            assert!(Ipv4::binary_to_decimal("0000000a").is_err());
            assert!(Ipv4::binary_to_decimal("abcdefgh").is_err());
        }      
    }    




























}

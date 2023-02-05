
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use  frc46_token::token::Token;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn rcgen_test() {
        extern crate rcgen;
        use rcgen::generate_simple_self_signed;
        let subject_alt_names = vec!["hello.world.example".to_string(),
                                     "localhost".to_string()];

        let cert = generate_simple_self_signed(subject_alt_names).unwrap();
// The certificate is now valid for localhost and the domain "hello.world.example"
        println!("{}", cert.serialize_pem().unwrap());
        println!("{}", cert.serialize_private_key_pem());
    }
}

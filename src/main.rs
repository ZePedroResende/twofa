
use std::{str::FromStr, time};
use hmac::{Hmac, Mac};
//use sha1::Sha1;
use totp_lite::{totp, totp_custom, Sha1, Sha512};

use std::time::{SystemTime, UNIX_EPOCH};


//#[derive(Debug, Clone)]
//struct Key {
//    name: String,
//    digits: u8,
//    bytes: Vec<u8>
//}
//
//#[derive(Debug)]
//struct KeyParseError;
//
//
//impl FromStr for Key {
//    type Err = KeyParseError;
//
//    fn from_str(s: &str) -> Result<Self, Self::Err> {
//        let splited_vector : Vec<&str> = s.split_whitespace().collect::<Vec<&str>>();
//
//        if splited_vector.len() != 3{
//            return Err(KeyParseError);
//        } 
//
//        let name = splited_vector[0].to_owned();
//        let size = splited_vector[1].parse::<u8>().map_err(|_| KeyParseError)?;
//        let bytes = splited_vector[2].as_bytes().to_vec();
//
//        Ok(Key{name, digits: size, bytes })
//    }
//}
//
//type HmacSha1 = Hmac<Sha1>;
//
//impl Key {
//
//    fn hotp (&self, key: Vec<u8>, counter: &[u8; 8]) -> String{
//      let mut mac = <HmacSha1 as Mac>::new_from_slice(&key).expect("failed to create hash");
//      HmacSha1::update(&mut mac, counter);
//      let hash = mac.finalize().into_bytes();
//      
//
//
//      let offset: usize = (hash.last().unwrap() & 0xf) as usize;
//      let binary: u64 = (((hash[offset] & 0x7f) as u64) << 24)
//          | ((hash[offset + 1] as u64) << 16)
//          | ((hash[offset + 2] as u64) << 8)
//          | (hash[offset + 3] as u64);
//  
//      format!("{:01$}", binary % (10_u64.pow(self.digits as u32)), self.digits as usize)
//    }
//
//    fn totp (&self, time: u64) -> String{
//        self.hotp(self.bytes.clone(), &counter(time))
//    }
//
//
//}
//
//    fn counter(time: u64) -> [u8; 8] {
//        to_bytes(time/30)
//    }
//
//    fn counter_with_step(time: u64, step: u64) -> [u8; 8] {
//        to_bytes(time/step)
//    }
//
//fn to_bytes(n: u64) -> [u8; 8] {
//    let mask = 0x00000000000000ff;
//    let mut bytes: [u8; 8] = [0; 8];
//    (0..8).for_each(|i| bytes[7 - i] = (mask & (n >> (i * 8))) as u8);
//    bytes
//}

fn main() {
//    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
//    let key =    "A 6 12345678901234567890".parse::<Key>().expect("Failed parse");
//    println!("{:?}", key);
//    let out = key.totp(time);

// Negotiated between you and the authenticating service.
let password: &[u8] = b"IBNXDO424423GF4A";

// The number of seconds since the Unix Epoch.
let seconds: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

// Specify the desired Hash algorithm via a type parameter.
// `Sha1` and `Sha256` are also available.
let result: String = totp_custom::<Sha1>(30, 6,password, seconds);

    println!("{:?} ",result);
}

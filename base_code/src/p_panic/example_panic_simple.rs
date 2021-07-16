use std::net::IpAddr;

#[test]
fn test_ip() {
    let home_ip: IpAddr = "192.168.1.11".parse().unwrap();
    println!("{}", home_ip)
}

#[test]
fn test_guess() {
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100 , got {}", value);
            }
            Guess { value }
        }
        pub fn get_value(&self) -> i32 {
            self.value
        }
    }

    let g = Guess::new(32);
    let _v = g.get_value();
}

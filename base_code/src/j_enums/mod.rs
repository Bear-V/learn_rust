mod enums_example;

pub fn example() {
    let ip_type = enums_example::IpAddrKind::V4;
    enums_example::init_enums();
    enums_example::route(ip_type);
}

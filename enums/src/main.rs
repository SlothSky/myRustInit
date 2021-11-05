// The IpAddrType is afterwards a custom data type
#[derive(Debug)]
enum IpAddrType {
    V4, 
    V6
}

fn main() {
    // Instances of the IpAddrType:
    let four = IpAddrType::V4;
    let six = IpAddrType::V6;

    route(four);
    route(six);
}
// This function can now be called with all IpAddrType options:
fn route(ip_type: IpAddrType) {
    println!("We chose IP address type: {:?}.", ip_type);

}
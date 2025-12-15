/*
 * Creating an enum
 * should be all upper case
 * added derive to see for compile
 * as per usage in fn route
*/
#[derive(PartialEq)]
enum IpVersion {
    V4,
    V6,
}

/*
 * function that can take whole enum
*/
fn route(ip_kind: IpVersion) {
    if ip_kind == IpVersion::V4 {
        println!("Its version 4");
        return;
    }
    println!("its version 6");
}

fn main() {
    println!("Hello enum!");
    let four = IpVersion::V4;
    let six = IpVersion::V6;
    route(four);
    route(six);
    let using_struct = IpAnotherAddr {
        kind: IpVersion::V4,
        address: String::from("127.0.0.1"),
    };
    {
        println!("Checking struct {}", using_struct.address);
        println!("and its ip type below"); 
        route(using_struct.kind);
    }
    {
        let home = IpHandling::V4(String::from("123.0.0.23:443"));
        // println!("Can we print? -> {}", home.0);
        let loopback = IpHandling::V6(String::from("::1"));
    }
    {
        let home = IpSplit::V4(127, 0, 0, 1);
        let loopback = IpSplit::V6(String::from("::1"));
    }
}

/*
 * Multiple handling version from none to some values
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct Ipv4Addr {
    value: String,
}

struct Ipv6Addr {
    value: String,
}

/*
 * Standard library definitions for address enum
 * using the structs inside the enums
 * not bringed into scope so we can use it
*/
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// way to handle multiple values per enum
enum IpSplit {
    V4(u8, u8, u8, u8),
    V6(String),
}

/*
 * Smaller version handling struct like 
 * but just with an enum to set types
*/
enum IpHandling {
    V4(String),
    V6(String),
}

/*
 * handling an enum through structs
*/
struct IpAnotherAddr {
    kind: IpVersion,
    address: String,
}

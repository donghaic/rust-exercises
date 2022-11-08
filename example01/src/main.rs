

use enum_dispatch::*;

fn main() {
    println!("Hello, world!");

    let fb: Bidders = Facebook::new().into();
    let google: Bidders = Google::new().into();
    let bidders = vec![fb, google];
    println!("{:?}", bidders);

    for bidder in bidders {
        bidder.make_requests();
        bidder.make_bids();
    }
}

#[enum_dispatch(Bidders)]
trait Bidder {
    fn make_requests(&self);

    fn make_bids(&self);
}

struct AdSource {
    id: u64,
    uri: String,
}

struct HttpResponseData {
    status_code: u8,
    body: Vec<u8>,
}

struct HttpRequestData {
    method: String,
    uri: String,
    body: Vec<u8>,
}

#[enum_dispatch]
#[derive(Debug)]
enum Bidders {
    Google,
    Facebook,
}

#[derive(Debug)]
struct Google;

impl Google {
    fn new() -> Self {
        Google
    }
}

impl Bidder for Google {
    fn make_requests(&self) {
        println!("Google make_requests");
    }

    fn make_bids(&self) {
        println!("Google make_bids");
    }
}

#[derive(Debug)]
struct Facebook;

impl Facebook {
    fn new() -> Facebook {
        Facebook
    }
}

impl Bidder for Facebook {
    fn make_requests(&self) {
        println!("Facebook make_requests");
    }

    fn make_bids(&self) {
        println!("Facebook make_bids");
    }
}

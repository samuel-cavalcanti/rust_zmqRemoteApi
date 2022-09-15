use uuid::Uuid;
fn main() {
    println!("FLAG SNDMORE: {}", zmq::SNDMORE);
    println!("FLAG DONTWAIT: {}", zmq::DONTWAIT);

    let id = Uuid::new_v4();
    println!("id: {}", id);

    let data = b"hello";
    // lower case
    println!("{:x?}", data);
}

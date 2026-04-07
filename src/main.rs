mod output;
mod capture;

fn main() {
    println!("{}", output::motd::motd());

    

    // let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();
    // println!("{}", Device::lookup().unwrap().unwrap().name);
    // while let Ok(packet) = cap.next_packet() {
    //     println!("received packet! {:?}", packet);
    // }
}

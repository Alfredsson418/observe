// Handles output to terminal

use crate::capture;

pub fn p_devices() {
    let devices = capture::device::get_all_devices();
    for device in devices {
        if !device.flags.is_running() {
            continue;
        }
        print!("name: {}\n", device.name);

        if device.desc.is_some() {
            print!("    desc: {}\n", device.desc.unwrap());
        }

        for addr in device.addresses {
            if addr.addr.is_ipv4() {
                print!("    inet {}\n", addr.addr.to_string());
            } else if addr.addr.is_ipv6() {
                print!("    inet6 {}\n", addr.addr.to_string());
            } else {
                print!("    other {}\n", addr.addr.to_string());
            }

            if addr.netmask.is_some() {
                print!("            netmask {}\n", addr.netmask.unwrap());
            }
            if addr.broadcast_addr.is_some() {
                print!("            boardcast {}\n", addr.broadcast_addr.unwrap());
            }
            if addr.dst_addr.is_some() {
                print!("            dst_addr {}\n", addr.dst_addr.unwrap());
            }
        }

        print!("\n");
    }
}

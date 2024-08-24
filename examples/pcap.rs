use rusty_zigbee_sniffer::{devices::cc253x::CC253X, Pcap};
use std::cell::RefCell;
use std::env;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn main() {
    let args: Vec<_> = std::env::args().collect();

    let channel: u8 = if let Some(channel) = args.get(1) {
        channel
            .parse()
            .expect(&format!("could not parse channel: {:?}", channel))
    } else {
        15
    };

    let pcap_path = if let Some(path) = args.get(2) {
        PathBuf::from(path)
    } else {
        let mut pcap_path = PathBuf::from(env::var("HOME").expect("could not find home directory"));
        pcap_path.push(format!("{}.pcap", get_current_time_formatted()));
        pcap_path
    };

    println!("pcap writer channel: {:?}, path: {:?}", channel, pcap_path);

    let pcap = Rc::new(RefCell::new(
        Pcap::new(pcap_path.to_str().unwrap()).unwrap(),
    ));
    pcap.try_borrow_mut().unwrap().write_header().unwrap();

    let mut cc253x = CC253X::open(channel).unwrap();
    let on_packet = |frame: &[u8]| {
        println!("{:?} bytes", frame.len());
        pcap.try_borrow_mut().unwrap().write_record(frame).unwrap();
        Ok(())
    };
    let on_unknown_packet = |frame: &[u8]| {
        println!("!unknown frame! {:?}", frame);
        Ok(())
    };
    cc253x
        .blocking_sniff(&on_packet, Some(&on_unknown_packet))
        .unwrap();
}

fn get_current_time_formatted() -> String {
    let dt = SystemTime::now();
    format!(
        "{:?}",
        dt
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    )
}

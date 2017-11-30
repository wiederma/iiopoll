
use std::path::PathBuf;
use std::time::Duration;
use std::thread;
use sensor;


pub fn probe() {
    let iioroot = PathBuf::from(r"/sys/bus/iio/devices");
    let mut nodes = sensor::SensorNode::enumerate(&iioroot).unwrap();

    loop {
        for mut node in &mut nodes {
            let measurement = node.do_measurement();
            println!("{:?}", measurement);
            thread::sleep(Duration::from_millis(1000))
        }
    }
}

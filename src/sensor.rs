use std::io;
use std::io::Read;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use time;
use time::Timespec;

const TEMPERATURE_FILE: &str = "in_temp_input";
const HUMIDITY_RELATIVE_FILE: &str = "in_humidityrelative_input";
const PRESSURE_FILE: &str = "in_pressure_input";

#[derive(Debug)]
pub struct SensorNode {
    name: String,
    temp_endpoint: Option<fs::File>,
    humidity_rel_endpoint: Option<fs::File>,
    pressure_endpoint: Option<fs::File>,
}

impl SensorNode {
    fn new(path: &PathBuf) -> Self {
        let mut name = String::new();
        let mut name_file = fs::File::open(path.join("name")).unwrap();
        // unwrap() is fine. If this file is not there, there is something serioulsy wrong.
        name_file.read_to_string(&mut name).unwrap();

        // Try to open endpoints and store filedescriptor.
        let temp_endpoint = if path.join(TEMPERATURE_FILE).exists() {
            let fd = fs::File::open(path.join(TEMPERATURE_FILE));
            fd.ok()
        } else {
            None
        };

        let humid_rel_endpoint = if path.join(HUMIDITY_RELATIVE_FILE).exists() {
            let fd = fs::File::open(path.join(HUMIDITY_RELATIVE_FILE));
            fd.ok()
        } else {
            None
        };

        let pressure_endpoint = if path.join(PRESSURE_FILE).exists() {
            let fd = fs::File::open(path.join(PRESSURE_FILE));
            fd.ok()
        } else {
            None
        };

        Self {
            name: name.trim().to_string(),
            temp_endpoint: temp_endpoint,
            humidity_rel_endpoint: humid_rel_endpoint,
            pressure_endpoint: pressure_endpoint,
        }
    }

   pub fn enumerate(path: &PathBuf) -> io::Result<Vec<Self>> {
        let mut sensors = Vec::new();

        for entry in fs::read_dir(path)? {
            let sensor_root = entry?;
            let sensor = Self::new(&sensor_root.path());
            sensors.push(sensor);
        }

        Ok(sensors)
    }

    fn temperature(&mut self) -> Option<String> {
        if let Some(ref mut fd) = self.temp_endpoint {
            let mut buf = String::new();
            let ret = match fd.read_to_string(&mut buf) {
                Ok(_) => Some(buf.trim().to_string()),
                Err(_) => None,
            };

            return ret;
        }

        None
    }

    fn humidity_relative(&mut self) -> Option<String> {
        if let Some(ref mut fd) = self.humidity_rel_endpoint {
            let mut buf = String::new();
            let ret = match fd.read_to_string(&mut buf) {
                Ok(_) => Some(buf.trim().to_string()),
                Err(_) => None,
            };

            return ret;
        }

        None
    }

    fn pressure(&mut self) -> Option<String> {
        if let Some(ref mut fd) = self.pressure_endpoint {
            let mut buf = String::new();
            let ret = match fd.read_to_string(&mut buf) {
                Ok(_) => Some(buf.trim().to_string()),
                Err(_) => None,
            };

            return ret;
        }

        None
    }

    pub fn do_measurement(&mut self) -> Measurement {
        let measurement = Measurement::new(self);
        measurement
    }
}

#[derive(Debug)]
pub struct Measurement<'a> {
    sensor: &'a SensorNode,
    host: String,
    location: Option<String>,
    timestamp: Timespec,
    temperature: Option<String>,
    humidity_relative: Option<String>,
    pressure: Option<String>,
}

impl<'a> Measurement<'a> {
    fn new(sensor: &'a mut SensorNode) -> Self {
        let mut hostfd = File::open("/etc/hostname").unwrap();  // TODO
        let mut hostname = String::new();
        hostfd.read_to_string(&mut hostname);

        // Read out values now! Must do this here because of the borrow checker.
        let temperature = sensor.temperature();
        let humidity_relative = sensor.humidity_relative();
        let pressure = sensor.pressure();

        Self {
            sensor: sensor,
            host: hostname,
            location: None,
            timestamp: time::get_time(),
            temperature: temperature,
            humidity_relative: humidity_relative,
            pressure: pressure,
        }
    }

    fn location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }

    fn to_json() -> String {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    extern crate tempdir;

    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    use super::*;

    fn setup() -> tempdir::TempDir {
        let tmpdir = tempdir::TempDir::new("cargo-test").unwrap();

        // Run setup script. I am too lazy to do this in rust... For now! :)
        {
            let arg = tmpdir.path().to_str().unwrap();
            Command::new("scripts/setup.sh").arg(arg).output().unwrap();
        }

        tmpdir
    }

    fn teardown(tmpdir: PathBuf) {
        fs::remove_dir_all(tmpdir).unwrap();
    }

    #[test]
    fn device_enumeration() {
        let tmpdir = setup();
        let tmpdir_path_buf = tmpdir.path().to_path_buf();
        SensorNode::enumerate(&tmpdir_path_buf).unwrap();
        teardown(tmpdir_path_buf);
    }

    #[test]
    fn read_name() {
        let tmpdir = setup();
        let tmpdir_path_buf = tmpdir.path().to_path_buf();
        let sensors = SensorNode::enumerate(&tmpdir_path_buf).unwrap();
        for sensor in sensors {
            let name = sensor.name;
            assert_eq!(name, "HANS");
        }

        teardown(tmpdir_path_buf);
    }

    #[test]
    fn read_temperature() {
        let tmpdir = setup();
        let tmpdir_path_buf = tmpdir.path().to_path_buf();
        let sensors = SensorNode::enumerate(&tmpdir_path_buf).unwrap();
        for mut sensor in sensors {
            let temperature = sensor.temperature();
            if let Some(t) = temperature {
                assert_eq!(t, "42");
            }
        }

        teardown(tmpdir_path_buf);
    }

    #[test]
    fn read_humidity_relative() {
        let tmpdir = setup();
        let tmpdir_path_buf = tmpdir.path().to_path_buf();
        let sensors = SensorNode::enumerate(&tmpdir_path_buf).unwrap();
        for mut sensor in sensors {
            let humidity_relative = sensor.humidity_relative();
            if let Some(h) = humidity_relative {
                assert_eq!(h, "50");
            }
        }

        teardown(tmpdir_path_buf);
    }

    #[test]
    fn read_pressure() {
        let tmpdir = setup();
        let tmpdir_path_buf = tmpdir.path().to_path_buf();
        let sensors = SensorNode::enumerate(&tmpdir_path_buf).unwrap();
        for mut sensor in sensors {
            let pressure = sensor.pressure();
            if let Some(p) = pressure {
                assert_eq!(p, "666");
            }
        }

        teardown(tmpdir_path_buf);
    }

    #[test]
    fn create_measurement() {
        let tmpdir = setup();
        let tmpdir_path_buf = tmpdir.path().to_path_buf();
        let mut sensors = SensorNode::enumerate(&tmpdir_path_buf).unwrap();
        let measurement = sensors[0].do_measurement().location("Haus im Wald".to_string());

        // Don't know which sensor it picks. Let's check everything. :)
        if let Some(t) = measurement.temperature {
            assert_eq!(t, "42");
        }
        if let Some(h) = measurement.humidity_relative {
            assert_eq!(h, "50");
        }
        if let Some(p) = measurement.pressure {
            assert_eq!(p, "666");
        }

        assert_eq!(measurement.location, Some("Haus im Wald".to_string()));

        teardown(tmpdir_path_buf);
    }
}

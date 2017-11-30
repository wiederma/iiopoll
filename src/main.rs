extern crate time;
extern crate clap;

mod sensor;
// mod cli;
mod probe;

// TODO
// to configure:
// time intervall
// database url
// ring buffer size
// logging
fn main() {
    probe::probe();
}

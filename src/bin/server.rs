#[macro_use]
extern crate clap;
extern crate libmodbus_rs;

use clap::{App, Arg, ArgMatches};
use libmodbus_rs::{Modbus, ModbusClient, ModbusRTU, ModbusTCP, ModbusTCPPI};
use std::io::Error;


#[derive(Debug, Eq, PartialEq)]
enum Backend {
    TCP,
    TCPPI,
    RTU,
}

fn run(matches: &ArgMatches) -> Result<(), Box<Error>> {
    let backend = match matches.value_of("backend").unwrap() {
        "tcp" => Backend::TCP,
        "tcppi" => Backend::TCPPI,
        "rtu" => Backend::RTU,
        _ => unreachable!(), // because clap ensures that for us
    };

    let mut modbus = match backend {
        Backend::RTU => {
            let serial_interface = matches.value_of("serial_interface").unwrap(); // We can unwrap here because clap ensures via default_value that we have a default value here.
            let baud = value_t!(matches, "baud", i32).unwrap(); // Here we also can just unwrap, clap saves our aes here as well.
            let slave_id = value_t!(matches, "slave_id", u8).unwrap();  // dito clap default_value 247
            let mut modbus = Modbus::new_rtu(&serial_interface, baud, 'N', 8, 1).expect("Unable to create modbus RTU context");
            modbus.set_slave(slave_id).expect("Could not set slave id");
            modbus
        },
        Backend::TCP => {
            let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).expect("Could not create modbus TCP context");
            modbus
        },
        Backend::TCPPI => {
            let mut modbus = Modbus::new_tcp_pi("::1", "1502").expect("Could not create modbus TCPPI context");
            modbus
        },
    };

    if matches.is_present("debug") {
        modbus.set_debug(true).expect("Could not set DEBUG mode");
    }

    modbus.connect().expect("Could not connect");

    let mut dest = vec![0u16];
    modbus.read_registers(0, 2, &mut dest).expect("Could not read registers");

    Ok(())
}

fn main() {
    let matches = App::new("xmz-client")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Console Client for 'xMZ-Mod-Touch'-Platform")
        .author("Stefan Müller <s.mueller@it.kls-glt.de>")
        .arg(Arg::with_name("backend")
            .help("which backend sould be used")
            .long("backend")
            .short("b")
            .possible_values(&["rtu", "tcp", "tcppi"])
            .takes_value(true)
            .required(true)
            .default_value("rtu"))
        .arg(Arg::with_name("slave_id")
            .help("Modbus slave address")
            .long("slave_id")
            .short("i")
            .takes_value(true)
            .required(true)
            .default_value("247"))
        .arg(Arg::with_name("serial_interface")
            .help("which serial interface should be used")
            .long("serial_interface")
            .short("s")
            .takes_value(true)
            .required(true)
            .default_value("/dev/ttyUSB0"))
        .arg(Arg::with_name("baud")
            .help("baud rate")
            .long("baud")
            .takes_value(true)
            .required(true)
            .default_value("9600"))
        .arg(Arg::with_name("debug")
            .help("modbus debug mode")
            .long("debug"))
        .get_matches();

    if let Err(ref err) = run(&matches) {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}

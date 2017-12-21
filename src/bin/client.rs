#[macro_use] extern crate clap;
#[macro_use] extern crate failure;
extern crate libmodbus_rs;

use clap::{App, Arg, ArgMatches};
use failure::{ResultExt, Error};
use libmodbus_rs::{
    FunctionCode,
    Modbus,
    ModbusClient,
    ModbusRTU,
    ModbusTCP,
    ModbusTCPPI,
};


enum Backend {
    RTU,
    TCP,
    TCPPI,
}


fn run(matches: &ArgMatches) -> Result<(), Error> {
    let backend = match matches.value_of("backend").unwrap() {
        "tcp" => Backend::TCP,
        "tcppi" => Backend::TCPPI,
        "rtu" => Backend::RTU,
        _ => unreachable!(), // because clap ensures that for us
    };

    // Set SlaveID, clap ensures that slave_id is 247 if no slave_is was given by the user.
    let slave_id = if matches.is_present("slave_id") {
        value_t!(matches, "slave_id", u8)?
    } else { unreachable!() };

    // Data Bits
    let data_bits = value_t!(matches, "data_bits", i32)?;

    let mut modbus = match backend {
        Backend::RTU => {
            let serial_interface = matches.value_of("serial_interface").unwrap(); // We can unwrap here because clap ensures via default_value that we have a default value here.
            let baud = value_t!(matches, "baud", i32).unwrap(); // Here we also can just unwrap, clap saves our aes here as well.
            let mut modbus = Modbus::new_rtu(&serial_interface, baud, 'N', data_bits, 1).expect("Unable to create modbus RTU context");
            // modbus.rtu_set_serial_mode(SerialMode::RtuRS232).expect("Could not set RTU mode");
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

    // Check if Address was given
    let address = if matches.is_present("address") {
        value_t!(matches, "address", u16)?
    } else { 0 };

    // Check if value was given
    let value = if matches.is_present("value") {
        value_t!(matches, "value", bool)?
    } else { false };

    modbus.connect().expect("Could not connect");

    match matches.value_of("function_code").unwrap() {
        "0x01" | "1" | "read_bits"              => {
            let mut dest = vec![0u8; 10];
            modbus.read_bits(0, 4, &mut dest).context("Could not read bits")?;

            println!("{:?}", dest);
        },
        "0x02" | "2" | "read_input_bits"        => println!("read input bits"),
        "0x03" | "3" | "read_registers"         => println!("read registers"),
        "0x04" | "4" | "read_input_registers"   => {
            let mut dest = vec![0u16; Modbus::MAX_READ_REGISTERS as usize];
            modbus.read_input_registers(0, 4, &mut dest).context("Could not read input registers")?;

            println!("{:?}", dest);
        },
        "0x05" | "5" | "write_bit"              => {
            modbus.write_bit(address, value).context("Could not write single coil")?;
        },
        "0x08" | "8" | "diagnostic"                  => {
            // let mut raw_request = vec![slave_id, FunctionCode::Diagnostic as u8, 0x0000, 0xBEEF];
            // Neustart
            let mut raw_request = vec![slave_id, FunctionCode::Diagnostic as u8, 0x0001, 0x0034];
            let mut response = vec![0u8; Modbus::RTU_MAX_ADU_LENGTH];
            let len = raw_request.len();

            println!("send_raw_request {:?}", modbus.send_raw_request(&mut raw_request, len));
            match modbus.receive_confirmation(&mut response) {
                Ok(len) => {
                    println!("{} byte empfangen!", len);
                    println!("response: {:?}", response);
                },
                Err(_) => {}
            }
        },
        _ =>bail!("Unsupported function code"),
    };

    Ok(())
}

fn main() {
    let matches = App::new("xmz-client")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Console Client for 'xMZ-Mod-Touch'-Platform")
        .author("Stefan Müller <s.mueller@it.kls-glt.de>")
        .arg(Arg::with_name("address")
            .help("Modbus Address to request")
            .long("address")
            .takes_value(true)
            .short("a")
            .required(false))
        .arg(Arg::with_name("backend")
            .help("which backend sould be used")
            .long("backend")
            .short("b")
            .possible_values(&["rtu", "tcp", "tcppi"])
            .takes_value(true)
            .required(true)
            .default_value("rtu"))
        .arg(Arg::with_name("baud")
            .help("baud rate")
            .long("baud")
            .takes_value(true)
            .required(true)
            .default_value("9600"))
        .arg(Arg::with_name("data_bits")
            .help("Number of Data bits")
            .long("data_bits")
            .short("d")
            .possible_values(&["5", "6", "7", "8"])
            .takes_value(true)
            .required(true)
            .default_value("8"))
        .arg(Arg::with_name("debug")
            .help("modbus debug mode")
            .long("debug"))
        .arg(Arg::with_name("function_code")
            .help("Modbus Function Code")
            .index(1)
            .long("function_code")
            .short("fc")
            .required(true))
        .arg(Arg::with_name("value")
            .help("Modbus Value that should writen at address (a)")
            .long("value")
            .short("v")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("serial_interface")
            .help("which serial interface should be used")
            .long("serial_interface")
            .short("s")
            .takes_value(true)
            .required(true)
            .default_value("/dev/ttyUSB0"))
        .arg(Arg::with_name("stop_bits")
            .help("Number of Stop bits")
            .long("stop_bits")
            .possible_values(&["0", "1", "2"])
            .takes_value(true)
            .required(true)
            .default_value("1"))
        .arg(Arg::with_name("parity")
            .help("Parity bit (N = 'None', O = 'Odd', E = 'Even')")
            .long("parity")
            .short("p")
            .possible_values(&["N", "O", "E"])
            .takes_value(true)
            .required(true)
            .default_value("N"))
        .arg(Arg::with_name("slave_id")
            .help("Modbus slave address")
            .long("slave_id")
            .short("i")
            .takes_value(true)
            .required(true)
            .default_value("247"))

        .get_matches();

    if let Err(err) = run(&matches) {
        for cause in err.causes() {
            println!("{}", cause);
        }

        std::process::exit(1)
    }
}

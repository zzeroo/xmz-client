trait Outputs {
    fn set(&mut self, status: bool);
    fn status(&self) -> bool;
}

struct ShiftRegister;

struct MetzConnectDO4;

impl Outputs for ShiftRegister {
    fn set(&mut self, status: bool) {
    }

    fn status(&self) -> bool {
        false
    }
}

// Metz-Connect DO4 Modbus Modul mit 4 digitalen Ausgängen
impl Outputs for MetzConnectDO4 {
    fn set(&mut self, status: bool) {
    }

    fn status(&self) -> bool {
        false
    }
}

#[derive(Debug)]
struct Server<T>
    where T: Outputs + Clone
{
    leds: Vec<T>,
}

impl<T> Server<T>
    where T: Outputs + Clone
{
    fn new() -> Self {
        Server {
            leds: vec![ShiftRegister::new()],
        }
    }
}


fn main() {
    println!("Test Server Struktur");

    let server = Server::new();

    println!("{:?}", server);
}

// Testaufbau aller wichtiger Komponenten der xMZ-Mod-Touch Plattform
//
// Diese Datei sollte alle essentialen Komponenten der xMZ-Mod-Touch Server Komponente beinhalten.

// Realis (ShifrRegister), LEDs (ShiftRegister), Modbus Schaltmodule (MetzConnectDO4)
//
// Es ist noch nicht ganz klar ob dieses Trait wirklich gebraucht wird.
trait Outputs {
    fn set(&mut self, status: bool);
    fn status(&self) -> bool;
}

#[derive(Clone, Debug)]
struct ShiftRegister;
impl ShiftRegister {
    fn new() -> Self {
        ShiftRegister
    }
}

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
    leds: Vec<Box<T>>,
}

impl<T> Server<T>
    where T: Outputs + Clone
{
    fn new() -> Self {
        Server {
            leds: vec![],
        }
    }
}


fn main() {
    println!("Test Server Struktur");

    let server: Server<ShiftRegister> = Server::new();

    println!("{:?}", server);
}

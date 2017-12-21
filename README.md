# Client für die 'xMZ-Mod-Touch'-Plattform

Dieser Client kann dazu verwendet werden die verschiedenen Funktionen der xMZ-Mod-Touch Plattform zu testen und diese zu steuern.

## Beispiele

### Abfrage aller Register einers DIO4 Modules

```bash
cargo run -- -b rtu -s /dev/ttyUSB0 --slave_id 35 4
```

### Ausgang ein schalten

```bash
cargo run -- -b rtu -s /dev/ttyUSB0 --slave_id 36 --address 0 --value true 5
```

### Ausgang aus schalten

```bash
cargo run -- -b rtu -s /dev/ttyUSB0 --slave_id 36 --address 0 --value false 5
```

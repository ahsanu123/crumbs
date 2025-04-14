

# 🏯 Hokushime 

Hokushime is project to clone Hiroshe db9 to RS232 converter

**Used Component**
- SIPEX SP3243ECA (will use SP3243EB)
- FT232BL (will use FT232BL)
- ATMLH916 (will use AT24CXX series, i2c based eeprom memory chip)

## Notes 

- FT232 Family

**FT232BL** is newer version of **FT232BM**
  
```shell
Feature	FT232BL	FT232RL	FT230X	FT232H
USB Support	USB 2.0 FS	USB 2.0 FS	USB 2.0 FS	USB 2.0 HS
UART	Yes	Yes	Yes	Yes
SPI/I²C/JTAG	No	No	No	Yes (MPSSE)
EEPROM Support	External	Internal	Internal	Internal
Integrated Xtal	No	Yes	Yes	Yes
Voltage I/O	3.3V / 5V	3.3V / 5V	3.3V / 5V	3.3V
Packages	SSOP28, etc.	SSOP28, QFN	QFN, DFN	LQFP, QFN
Channels	1	1	1	1 (multi-mode)
Misc Features	Basic UART	Full USB-UART	Lower cost	Advanced, MPSSE
```

| Feature                | FT232BM         | FT232BL         | FT232RL         | FT232RQ         | FT232H             |
|------------------------|------------------|------------------|------------------|------------------|----------------------|
| USB Version            | USB 1.1          | USB 2.0 FS       | USB 2.0 FS       | USB 2.0 FS       | USB 2.0 HS           |
| Max Baud Rate          | 1 Mbps           | 3 Mbps           | 3 Mbps           | 3 Mbps           | 12 Mbps              |
| Package                | SSOP28           | SSOP28           | SSOP28, QFN32    | QFN32            | QFN32, LQFP48        |
| Internal EEPROM        | ❌ No             | ❌ No             | ✅ Yes (128 B)    | ✅ Yes (128 B)    | ✅ Yes (configurable) |
| Internal Oscillator    | ❌ No             | ❌ No             | ✅ Yes            | ✅ Yes            | ✅ Yes                |
| Requires Crystal       | ✅ Yes (6 MHz)    | ✅ Yes (6 MHz)    | ❌ No             | ❌ No             | ❌ No                 |
| Voltage Supply         | 5V               | 5V               | 3.3V–5.25V       | 3.3V–5.25V       | 1.8V–5.25V           |
| IO Voltage             | 5V               | 5V               | 1.8V–3.3V (5V tolerant) | 1.8V–3.3V (5V tolerant) | 1.8V–3.3V (5V tolerant) |
| USB Battery Charging Detection | ❌ No      | ❌ No             | ✅ Yes            | ✅ Yes            | ✅ Yes                |
| Driver Support         | VCP, D2XX        | VCP, D2XX        | VCP, D2XX        | VCP, D2XX        | VCP, D2XX, MPSSE      |
| MPSSE (for SPI/I2C/JTAG) | ❌ No          | ❌ No             | ❌ No             | ❌ No             | ✅ Yes                |
| Target Application     | Basic USB-UART   | USB-UART         | USB-UART         | USB-UART         | USB Hi-Speed Serial / MPSSE |

# Rusty Zifbee Sniffer

Currently supports sniffing CC2531 flashed with sniffing firmware, based on https://github.com/homewsn/whsniff

## Pcap capture example

```bash
cargo run --example pcap -- 16
```

where 16 is the channel. You can also pass the file path as the second argument:

```bash
cargo run --example pcap -- 16 /home/$USER/sniff.pcap
```

### TODOS:

- Verify pcap capture and add stdout support for wireshark live sniffing
- Fix arbitrary length ignores
- Add support for docker one click install and maybe also sniffer firmware flashing
- Add support for other dongles
- Add trait for sniffing

# Dongle database

| Dongle | Description                                                                                        |
|--------|----------------------------------------------------------------------------------------------------|
| CC2530 | Zigbee and IEEE 802.15.4 wireless MCU with 256kB Flash and 8kB RAM                                 |
| CC2531 | Zigbee and IEEE 802.15.4 wireless MCU with up to 256kB Flash and 8kB RAM                           |
| CC2533 | A True System-on-Chip Solution for 2.4-GHz IEEE 802.15.4 and ZigBee Applications                   |
| CC2538 | 32-bit Arm Cortex-M3 Zigbee, 6LoWPAN, and IEEE 802.15.4 wireless MCU with 512kB Flash and 32kB RAM |
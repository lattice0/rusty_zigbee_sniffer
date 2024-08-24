# Rusty Zifbee Sniffer

Currently supports sniffing CC2541X flashed with sniffing firmware, based on https://github.com/homewsn/whsniff

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
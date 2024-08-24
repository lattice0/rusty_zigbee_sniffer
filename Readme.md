# Rusty Zifbee Sniffer

Currently supports sniffing CC2541X flashed with sniffing firmware, based on https://github.com/homewsn/whsniff

## Pcap capture example

ˋˋˋbash
cargo run --example pcap -- 16
ˋˋˋ

where 16 is the channel. You can also pass the file path as the second argument:

ˋˋˋbash
cargo run --example pcap -- 16 /home/$USER/sniff.pcap
ˋˋˋ

### TODOS:

- Verify pcap capture and add stdout support for wireshark live sniffing
- Fix arbitrary length ignores
- Add support for docker one click install and maybe also sniffer fimrware flashing
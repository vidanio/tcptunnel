# UDP -> TCP -> UDP tunnel

## Usage

``` $
# Start the tunnel on the tcp -> udp side
$ tcptunnel -u <ip:port> -t <ip:port>

# Start the tunnel on the udp -> tcp side
$ tcptunnel -u <ip:port> -t <ip:port> -s
```


## Compile

cd tcptunnel/src  
cargo build --release  
strip -s ../target/release/tcptunnel  


## Status

- [x] UDP -> TCP
- [x] TCP -> UDP
- [x] cli

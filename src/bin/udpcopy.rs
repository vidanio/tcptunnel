use std::time::{Duration, Instant};

use anyhow::Result;
use futures::stream::{StreamExt, TryStreamExt};
use tokio_util::codec::BytesCodec;
use tokio_util::udp::UdpFramed;

use tcptunnel::{to_endpoint, EndPoint};

use clap::Parser;



#[derive(Debug, Parser)]
#[clap(name = "udpcopy")]
struct Opt {
    /// Input source url
    /// It supports the following query parameters
    /// multicast=<ipv4_interface or ipv6_index>
    /// multicast_ttl=<u32> (IPv4-only)
    /// multicast_hops=<u32> (IPv6-only)
    /// buffer=<usize>
    #[clap(long, short, value_parser = to_endpoint)]
    input: EndPoint,
    /// Output source
    /// It supports the following query parameters
    /// multicast=<ipv4_interface or ipv6_index>
    /// multicast_ttl=<u32> (IPv4-only)
    /// multicast_hops=<u32> (IPv6-only)
    /// buffer=<usize>
    #[clap(long, short, value_parser = to_endpoint)]
    output: EndPoint,
    /// Verbose logging
    #[clap(long, short)]
    verbose: bool,
}

impl Opt {
    fn input_endpoint(&self) -> anyhow::Result<UdpFramed<BytesCodec>> {
        let e = &self.input;

        e.make_input()
    }

    fn output_endpoint(&self) -> anyhow::Result<UdpFramed<BytesCodec>> {
        let e = &self.output;

        e.make_output()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::parse();

    let udp_stream = opt.input_endpoint()?;
    let udp_sink = opt.output_endpoint()?;
    let udp_addr = opt.output.addr;

    let mut now = Instant::now();
    let mut size: usize = 0;

    let read = udp_stream.map_ok(move |(msg, _addr)| {
        let elapsed = now.elapsed();
        if elapsed > Duration::from_secs(1) {
            eprint!(
                "bps {:} last packet size {}\r",
                (size as f32 / elapsed.as_millis() as f32) * 8000f32,
                msg.len()
            );
            now = Instant::now();
            size = 0;
        } else {
            size += msg.len();
        }
        (msg.freeze(), udp_addr)
    });

    read.forward(udp_sink).await?;

    Ok(())
}

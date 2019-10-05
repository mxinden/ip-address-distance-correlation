use futures::prelude::*;
use rand::prelude::*;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::timer::Interval;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let own_public_ip: Ipv4Addr = args
        .get(1)
        .expect("provided own public ip address")
        .parse()
        .expect("valid ipv4 address");

    let fut = tokio_ping::Pinger::new()
        .map_err(|e| panic!("{:?}", e))
        .and_then(move |pinger| spawn_random_ping_on_interval(pinger.clone(), own_public_ip));

    tokio::run(fut.map_err(|e| panic!("{:?}", e)));
}

fn spawn_random_ping_on_interval(
    pinger: tokio_ping::Pinger,
    own_public_ip: Ipv4Addr,
) -> Box<dyn Future<Item = (), Error = ()> + Send> {
    let interval = Interval::new_interval(Duration::from_millis(10));

    Box::new(
        interval
            .for_each(move |_| {
                spawn_random_ping(pinger.clone(), own_public_ip);
                Ok(())
            })
            .map_err(|e| {
                eprintln!("{:?}", e);
            }),
    )
}

fn spawn_random_ping(pinger: tokio_ping::Pinger, own_public_ip: Ipv4Addr) {
    let rand_ip = Ipv4Addr::from(thread_rng().gen::<u32>());
    let ping: tokio_ping::PingFuture = pinger.chain(IpAddr::V4(rand_ip)).send();

    tokio::spawn(
        ping.and_then(move |resp| {
            if let Some(delay) = resp {
                println!(
                    "{:?};{:?}",
                    leading_bits_in_common(own_public_ip, rand_ip),
                    delay.as_millis()
                );
            }

            Ok(())
        })
        .map_err(|e| {
            eprintln!("{:?}", e);
        }),
    );
}

fn leading_bits_in_common(a: Ipv4Addr, b: Ipv4Addr) -> u32 {
    let xor: u32 = Into::<u32>::into(a) ^ Into::<u32>::into(b);

    xor.leading_zeros()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn returns_32_for_equal_addresses() {
        assert_eq!(
            32,
            leading_bits_in_common(Ipv4Addr::LOCALHOST, Ipv4Addr::LOCALHOST)
        );
    }

    #[test]
    fn returns_31_for_zero_and_one() {
        assert_eq!(
            31,
            leading_bits_in_common(Ipv4Addr::from(0), Ipv4Addr::from(1))
        );
    }

    #[test]
    fn returns_30_for_two_and_one() {
        assert_eq!(
            30,
            leading_bits_in_common(Ipv4Addr::from(2), Ipv4Addr::from(1))
        );
    }
}

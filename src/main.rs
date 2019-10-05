use futures::prelude::*;
use rand::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let own_public_ip: std::net::Ipv4Addr = args
        .get(1)
        .expect("provided own public ip address")
        .parse()
        .expect("valid ipv4 address");

    let fut = tokio_ping::Pinger::new().and_then(move |pinger| {
        let interval = tokio::timer::Interval::new_interval(std::time::Duration::from_millis(10));
        let pinger = pinger.clone();

        interval
            .for_each(move |_| {
                let rand_ip = std::net::Ipv4Addr::from(thread_rng().gen::<u32>());
                let fut: tokio_ping::PingFuture = pinger
                    .clone()
                    .chain(std::net::IpAddr::V4(rand_ip.clone()))
                    .send();

                tokio::spawn(
                    fut.and_then(move |resp| {
                        match resp {
                            Some(delay) => {
                                println!(
                                    "{:?}; {:?}",
                                    leading_bits_in_common(own_public_ip, rand_ip),
                                    delay.as_millis()
                                );
                            }
                            None => (),
                        }
                        Ok(())
                    })
                    .map_err(|e| panic!("{:?}", e)),
                );

                Ok(())
            })
            .map_err(|e| panic!("{:?}", e))
    });

    tokio::run(fut.map_err(|e| panic!("{:?}", e)));
}

fn leading_bits_in_common(a: std::net::Ipv4Addr, b: std::net::Ipv4Addr) -> u32 {
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
            leading_bits_in_common(std::net::Ipv4Addr::LOCALHOST, std::net::Ipv4Addr::LOCALHOST)
        );
    }

    #[test]
    fn returns_31_for_zero_and_one() {
        assert_eq!(
            31,
            leading_bits_in_common(std::net::Ipv4Addr::from(0), std::net::Ipv4Addr::from(1))
        );
    }

    #[test]
    fn returns_30_for_two_and_one() {
        assert_eq!(
            30,
            leading_bits_in_common(std::net::Ipv4Addr::from(2), std::net::Ipv4Addr::from(1))
        );
    }
}

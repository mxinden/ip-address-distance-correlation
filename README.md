ip-address-distance-correlation

The IPv4 address space was partitioned by the IANA to each regional internet
registry as blocks. Under the assumption that traffic speed correlates with
locality, this project tries to proof that **the number of common leading bits
of two ip addresses correlates with their ping latency**.

Using the distance between two ip addresses as a metric could be useful in a
gossip-style overlay network to decide which nodes to connect to to build an
efficient topology.

![Scatter plot](./scatter-plot.svg)

target/debug/ip-address-distance-correlation:
	cargo build
	sudo setcap cap_net_raw=ep target/debug/ip-address-distance-correlation

scatter-plot.svg: data.csv scatter-plot.gpi
	gnuplot scatter-plot.gpi

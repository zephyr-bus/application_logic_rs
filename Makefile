all: clean header_gen lib 

lib:
	cargo build --package application_logic_rs --target thumbv7em-none-eabi --release --no-default-features

header_gen:
	cbindgen --config cbindgen.toml --crate application_logic_rs --output generated/application_logic_rs.h --lang C

clean:
	rm -rf generated 
	cargo clean

clean_zephyr:
	rm -rf build

build_zephyr:
	west build -b nrf52840dk_nrf52840

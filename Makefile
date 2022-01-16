all: clean header_gen lib build_sample run

lib:
	cargo build --package application_logic_rs --target thumbv7em-none-eabi --release --no-default-features

expand:
	cargo +nightly expand --package application_logic_rs --target thumbv7em-none-eabi --release --no-default-features

header_gen:
	cbindgen --config cbindgen.toml --crate application_logic_rs --output generated/application_logic_rs.h --lang C

clean:
	rm -rf generated
	rm -rf tests/sample_project_zephyr/build
	cargo clean

build_sample:
	west build -b nrf52840dk_nrf52840 --pristine=always tests/sample_project_zephyr

run:
	renode tests/sample_project_zephyr/renode_setup_nrf52840.resc

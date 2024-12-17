#/bin/bash
mkdir target/flash
cd target/flash
rm *
cargo objcopy --release -- -O ihex firmware.hex
nrfutil pkg generate --hw-version 52 --sd-req=0x00  --application firmware.hex --application-version 1 firmware.zip
nrfutil device program --firmware firmware.zip --traits nordicDfu


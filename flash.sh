#!/bin/bash
set -e

: "${CPU:=atmega328p}"
: "${BAUD_RATE:=57600}"
: "${BIN_DIR:=/Applications/Arduino.app/Contents/Java/hardware/tools/avr/bin}"
: "${ETC_DIR:=/Applications/Arduino.app/Contents/Java/hardware/tools/avr/etc}"
: "${ELF:=target/avr-atmega328p/release/twostep.elf}"

main() {
  if [ "$#" -lt 1 ]; then
    echo "[ERROR] Missing argument device (e.g.: /dev/cu.wchusbserial1420)"
    exit 1
  fi

  if [ "$1" == "-b" ]; then
    cargo +nightly-2021-01-07 build --color=always -Z build-std=core --target avr-atmega328p.json --release
    #    cargo build --release
    shift
  fi

  local device
  device=$1
  shift
  $BIN_DIR/avrdude -v -C${ETC_DIR}/avrdude.conf \
    -p${CPU} \
    -carduino \
    -P${device} \
    -b${BAUD_RATE} \
    -D \
    $@ \
    -Uflash:w:${ELF}:e
}

main $@

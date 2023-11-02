
[Awesome ESP Rust](https://github.com/esp-rs/awesome-esp-rust)

# ESP32-C3

vidio tutorial https://www.youtube.com/playlist?list=PLkch9g9DEE0Lkm1LqcD7pZNDmXEczOo-a
tutorial - https://github.com/shanemmattner/ESP32-C3_Rust_Tutorials/tree/main/Tutorials/p0-output

- [esp-template](https://github.com/esp-rs/esp-template)          - no_std template.
- [esp-idf-template](https://github.com/esp-rs/esp-idf-template)  - std template.
     

Read [The Rust on ESP Book](https://esp-rs.github.io/book/introduction.html) it's a better starting point.

**template for all ESP32** https://github.com/esp-rs/esp-idf-template

    cargo generate https://github.com/esp-rs/esp-idf-template --name esp32-c3-examples
                     

## Install

    sudo apt-get update
    sudo apt-get install git wget flex bison gperf python3 python3-pip python3-venv cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0
     
    # update cmake                                      
    sudo apt-get remove cmake
    pip install cmake --upgrade

    cargo install espu
    espup install
    . $HOME/export-esp.sh

    cargo install cargo-generate
    cargo install espflash
    cargo install espmonitor
    cargo install ldproxy
          

## Build and Flash

Note first compilation takes huge time.

    cargo run --example blink
     

## Board used

[ESP-C3-01M-Kit](https://docs.ai-thinker.com/_media/esp32/docs/esp-c3-01m-kit-v1.0_specification.pdf)
      
**OMG it's alive**

## Building

### Prerequisites

1. Make sure you have the following dependencies installed:

   * `g++` 5.1 or later or `clang++` 3.5 or later
   * `python` 3 or 2.7
   * GNU `make` 3.81 or later
   * `cmake` 3.4.3 or later
   * `ninja`
   * `curl`
   * `git`
   * `ssl` which comes in `libssl-dev` or `openssl-devel`
   * `pkg-config` if you are compiling on Linux and targeting Linux

2. Clone the repo with submodules:

``` sh
git clone --recurse-submodules <git>
```

### Installing toolchain

You'll need to grab the latest xtensa-esp32s2-elf toolchain from [here](https://github.com/espressif/crosstool-NG/releases).

*Note:* Be sure to grab the the `esp32s2` one as well as right one for your machine (cpu and operating system); it will most likely be:
1. For intel CPUs: xtensa-esp32s2-elf-gcc8_4_0-esp-<version>-<os>-i686.tar.gz
2. For cpus CPUs: xtensa-esp32s2-elf-gcc8_4_0-esp-<version>-<os>-amd64.tar.gz

``` sh
mkdir ./esp
tar -xzf xtensa-esp32s2-elf-gcc8_4_0-esp-<version>-<os>-<arch>.tar.gz -C ./esp
```

#### Building and Flashing tools

``` sh
cargo install cargo-xbuild
cargo install cargo-espflash
# For python alternative (not recommended) use:
# pip install esptool 
```

### Configuring

rust-xtensa requires some setup which can be done by running the configure script and `setenv`:

``` sh
./configure
./setenv # Setup variables for cross-compiling xtensa targets
```


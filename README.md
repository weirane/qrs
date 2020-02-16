# qrs
A terminal QR code generator based on [qrcode-rust].

[qrcode-rust]: https://github.com/kennytm/qrcode-rust

### Installation

    cargo install --git https://github.com/weirane/qrs.git

### Usage

    qrs [-c COUNT]

Use `-c` to specify how many bytes of data will be read at most, defaults to 1024.

# nostrkeytool
Tool for NOSTR key operations

## Installation

You can install `nostrkeytool` using Cargo, the Rust package manager.

If you have Rust and Cargo installed, run the following command:

```bash
cargo install nostrkeytool
```

This command will download and compile `nostrkeytool`, making it available in your Cargo binary path.

Make sure `$HOME/.cargo/bin` is in your system's PATH to run `nostrkeytool` from anywhere.

## Usage

```plaintext
Usage: nostrkeytool [OPTIONS]

Options:
      --sec2npub <sec2npub>        Input: hex secret, Output: npub
      --sec2pubkey <sec2pubkey>    Input: hex secret, Output: pubkey
      --sec2nsec <sec2nsec>        Input: hex secret, Output: nsec
      --nsec2sec <nsec2sec>        Input: nsec, Output: hex secret
      --pubkey2npub <pubkey2npub>  Input: pubkey, Output: npub
      --npub2pubkey <npub2pubkey>  Input: npub, Output: pubkey
      --gen                        Generate a secret
  -h, --help                       Print help
  -V, --version                    Print version
```

## Contributing

We welcome contributions to nostrkeytool! Here's how to get started:

- Fork the project on GitHub.
- Create a new branch for your feature or bug fix.
- Write code and add tests for your changes.
- Ensure that all tests pass.
- Submit a pull request against the main branch.

Please follow the Rust coding conventions and include appropriate documentation.

## License

`nostrkeytool` is provided under the [MIT License](https://github.com/dcadenas/nostrkeytool/blob/master/LICENSE). See the LICENSE file for more details.

## Author

[Daniel Cadenas](https://github.com/dcadenas)

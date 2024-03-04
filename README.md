# CSS-2324

This repo contains my very dirty and frankly unfinished bonus exercise submission.

There are no guarantees being made about anything, especially correctness.

## Usage

This is just my solution which was never intended to be published and therefore expects the user to have [nix](https://nixos.org/download#download-nix) set up and the experimental features `nix-command` and `flakes` enabled.

Use `nix develop` in the project root or a subdirectory of it to enter the development shell, optionally append `-c $SHELL`. This will add a usable rust toolchain to your path which you can use to develop and compile the code.

Enter the two required numbers in the `main.rs` file and then use `cargo r` to execute the program. Comment out the things you don't need.

Use `nix run .#ex_8 -- message.txt message.enc.txt` to automatically encrypt the file `message.txt` using the provided public openssl key and write the result to `message.enc.txt`. You don't need to be in a development shell for this.

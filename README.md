# Nim

A small Rust terminal program for sending encrypted messages to a friend. Each person generates a key pair, swaps public keys with the other, and from then on they can encrypt and decrypt messages.

We started this project to practice Rust. It hasn't been audited, so please don't use it to protect anything sensitive.

## How it works

1. Each person generates a key pair (one private, one public).
2. You swap public keys. Your private key never leaves your machine.
3. Using your private key and the other person's public key, you both end up with the same shared secret (X25519).
4. That secret encrypts the message with ChaCha20-Poly1305, which also notices if anyone tampered with the text on the way.

Encrypted messages are printed in hexadecimal, so you can copy and paste them into a chat.

## Installation

You need [Rust](https://rustup.rs). Then:

```bash
git clone https://github.com/Proozz321/Nim.git
cd Nim
cargo run
```

## Usage

When you run the program, you get a menu:

```
[1] - Generate Private and Public keys
[2] - Encrypt your own message
[3] - Decrypt your friend's message
[4] - Exit
```

Here's how Alice and Bob would use it:

1. They each pick option 1 and save their keys.
2. They swap public keys.
3. Alice picks option 2, pastes her private key and Bob's public key, types a message, and sends Bob the hex text that comes out.
4. Bob picks option 3, pastes his private key, Alice's public key and the text he received, and reads the message.

## Structure

- `src/main.rs`: the menu and input handling
- `src/keys.rs`: key generation and hex conversion
- `src/crypto.rs`: encrypting and decrypting

## Limitations

- Keys aren't saved to a file, so you have to copy them yourself.
- There's no way to check that the public key you received really belongs to your friend.
- The shared secret is used directly as the cipher key, without going through a KDF.

## Authors

Keo ([@keohhh](https://github.com/keohhh))
Proozz ([@Proozz321](https://github.com/Proozz321))
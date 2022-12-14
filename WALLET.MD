<p align="left">
	<a href="http://tesseract.one/">
		<img alt="Tesseract" src ="./.github/logo-horiz.svg" height=128/>
	</a>
</p>

# Tesseract on Android for Wallet developers

Integration of Tesseract into your wallet allows any dApp to request your wallet to sign a transaction. Currently Tesseract enables native dApps to integrate with wallets through IPC, which from the user perspective is just a modal screen from the wallet.

## Getting started

Getting **Tesseract** to work in Android wallet is different from everywhere else only by the transports set it supports.

Currently we provide IPC transport, which allows the wallets to present theit screens on top of Android applications on request and sign the transactions.

### Initialize the transport

Such a transport can be created simply by providing `JNIEnv` reference to the transport:

```rust
tesseract_ipc_android::service::Transport::default(&env)
```

The rest stays exactly the same as it is described in the main repo of Tesseract ([THIS ONE](https://github.com/tesseract-one/Tesseract.rs/)) on this page [HERE](https://github.com/tesseract-one/Tesseract.rs/blob/master/docs/SERVICE.MD).

### Set up Rust

To add Rust, to your Wallet, please consider going through our guide [Setting up Rust](./RUST.MD). It contains the steps required to add Rust support to an Android app + some useful interop utils description we've built.

## Usage

* [Main Rust API documentation](https://github.com/tesseract-one/Tesseract.rs)
* [dApp developers documentation](./README.MD)

Once we publish the Kotlin wrappers, the doc will appear here.

## Examples

You can find the examples (**Demo Wallet** and a **Demo dApp**) in this repo [HERE](./examples).

## Installation

### Prerequisites
* Install your Rust environment: https://www.rust-lang.org/tools/install
* Download Android Studio: https://developer.android.com/studio
* For Rust we suggest VS Code: https://code.visualstudio.com/
* Android NDK (no need for CMAKE): https://developer.android.com/studio/projects/install-ndk#default-version

### On the Rust side you might need:

```toml
interop_android = { path = "../../rust/interop" } //useful interops we've created to easier interact with java

tesseract_ipc_android = { path = "../../rust/ipc", features=["service"]}
tesseract = {git = "https://github.com/tesseract-one/Tesseract.rs", branch="master", features=["service"]}
```

### On the side of Kotlin:

```gradle
implementation project(':java:tesseract-ipc')
implementation project(':java:tesseract-ipc-service')
implementation project(':java::interop-rust')

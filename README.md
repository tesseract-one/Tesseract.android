<p align="center">
	<a href="http://tesseract.one/">
		<img alt="Tesseract" src ="./.github/logo-vert.svg" height=256/>
	</a>
</p>

### **Tesseract Android** is an implementation of [Tesseract](https://github.com/tesseract-one/) protocol for Android OS. [Tesseract](https://github.com/tesseract-one/) seamlessly integrates dApps and wallets, regardless of the blockchain protocol.

#### **Tesseract** aims to improve the usability of the dApps without compromising security or decentralization.

This page is about specifics of running Tesseract on Android. If you need general info or Tesseract for another platform, please consider one of the following:
* [General info](https://github.com/tesseract-one/)
* [Tesseract for iOS (Swift)](https://github.com/tesseract-one/Tesseract.swift)
* [Tesseract shared Core (Rust)](https://github.com/tesseract-one/Tesseract.rs)


## Getting started

**Tesseract** is implemented in Rust and currently doesn't provide the Java/Kotlin wrappers, thus requiring some Rust coding. In the future, we aim to change it by offering wrappers, eliminating the need for any Rust code.

### Set up Rust

To add Rust, to your dApp or Wallet, please consider going through our guide [Setting up Rust](./RUST.MD). It contains the steps required to add Rust support to an Android app + some useful interop utils description we've built.

### Initialize Tesseract Client

Once the Rust part is set up, you can proceed to setting up Tesseract:

```rust
use tesseract_client;

let tesseract = tesseract_client::Tesseract::new(
	tesseract_client::delegate::SingleTransportDelegate::arc(),
).transport(TransportIPCAndroid::new(&env, application));
```

The initialization of Tesseract is essentially the same as it is described in the [Tesseract shared Core](tesseract-one/Tesseract.rs) except that to connect to a wallet via Tesseract, we need to specify the IPCTransport:

```rust
.transport(TransportIPCAndroid::new(&env, application))
```

where `env` is reference to the JNI environment and `application` is a reference to the Android Application.

#### Passing references from Kotlin/Java:

The easiest way to call Rust from Kotlin is to create a native JNI function:
```kotlin
private external fun rustInit(application: Application)
```

#### On the Rust side:

```rust
use jni_fn::jni_fn;

#[jni_fn("one.tesseract.example.app.RustCore")] //has to point to your actuall class in Kotlin
pub fn rustInit<'a>(env: JNIEnv<'a>, core: JObject<'a>, application: JObject<'a>) {
	//your initialization here
}
```

The rest of Tesseract APIs stay exacly the same everywhere. Please, consider to go through the docs in our [Tesseract shared Core](https://github.com/tesseract-one/Tesseract.rs) repo.

## Usage

* [Main Rust API documentation](https://github.com/tesseract-one/Tesseract.rs)
* [Wallet developers documentation](./WALLET.MD)

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

tesseract_ipc_android = { path = "../../rust/ipc", features=["client"]}
tesseract = {git = "https://github.com/tesseract-one/Tesseract.rs", branch="master", features=["client"]}
```

### On the side of Kotlin:

```gradle
implementation project(':java:tesseract-ipc')
implementation project(':java:tesseract-ipc-client')
implementation project(':java::interop-rust')
```

### Setting up Rust interop:

Please, consider the guide [HERE](./RUST.MD).

## Roadmap

- [x] v0.1 - IPC transport for Android - connect dApp/Wallet on the same device
- [x] v0.2 - demo dApp and Wallet
- [ ] v1.0 - Kotlin wrapper for Rust

## Changelog

* v0.2 - Created demo dApp and Wallet
* v0.1 - Created transport to connect dApp and Wallet

## License

Tesseract.android can be used, distributed and modified under [the Apache 2.0 license](LICENSE).

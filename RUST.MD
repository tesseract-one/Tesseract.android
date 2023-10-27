<p align="left">
	<a href="http://tesseract.one/">
		<img alt="Tesseract" src ="./.github/logo-horiz.svg" height=128/>
	</a>
</p>

# Adding Rust to your Android App

Create a new Android application or open your existing project.

Modify your `build.gradle`, by adding the following:
* `ndkVersion "24.0.8215888"` to your `android` section (or any other version that corresponds to your NDK)
* add `apply plugin: 'org.mozilla.rust-android-gradle.rust-android'` to enable the rust building
* the following section will make rust building automatic
```
tasks.whenTaskAdded { task ->
    if ((task.name == 'javaPreCompileDebug' || task.name == 'javaPreCompileRelease')) {
        task.dependsOn 'cargoBuild'
    }
}
```
* describe your rust dylib (this is how you build your Rust)
```
cargo {
    module  = "native"       // Or whatever directory contains your Cargo.toml
    libname = "app"          // Or whatever matches Cargo.toml's [package] name.
    targets = ["arm", "x86", "arm64", "x86_64"]  // See bellow for a longer list of options
    targetDirectory = 'native/target' //or wgatever directory contains your target directory
    rustupChannel = "nightly"
}
```
* add folloing dependencies:
```
implementation project(':java:tesseract-ipc')
implementation project(':java:tesseract-ipc-client')
implementation project(':java::interop-rust')
```
* register the following build plugin
```
id "org.mozilla.rust-android-gradle.rust-android" version "0.9.3" apply false
```

Create a rust library:
* `cargo new native --lib`
* Add the dependencies:
```toml
interop_android = {git = "https://github.com/tesseract-one/Tesseract.android", branch="master", features=["client"]}
tesseract_ipc_android = {git = "https://github.com/tesseract-one/Tesseract.android", branch="master", features=["client"]}
tesseract = {git = "https://github.com/tesseract-one/Tesseract.rs", branch="master", features=["client"]}
tesseract-protocol-test = {git = "https://github.com/tesseract-one/Tesseract.rs", branch="master", features=["client"]}
```
* specify the type of library
```
[lib]
crate_type = ["cdylib"]
```

Load the native library. For that add the following somewhere in your application (Application.kt or MainActivity.kt might work well enough):
```kotlin
companion object {
	init {
		System.loadLibrary("native")
	}
}
```

Now you can create your JNI functions, initializa Tesseract and start calling the Wallet.

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

Following rust targets must be innstalled:
```bash
rustup +nightly target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

On Mac you might want to point to a python you have installed by specifying
```properties
rust.pythonCommand=python3
```
in `local.properties`.

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


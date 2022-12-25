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

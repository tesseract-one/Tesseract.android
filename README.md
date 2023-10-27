<p align="center">
	<a href="http://tesseract.one/">
		<img alt="Tesseract" src ="./.github/logo-vert.svg" height=256/>
	</a>
</p>

### **Tesseract Android** provides Java/Kotlin APIs for [Tesseract](https://github.com/tesseract-one/), a dApp-Wallet bridge designed to make dApp/wallet communication on mobile devices simple and natural without compromising decentralization and security

If you are looking for Tesseract docs for another language/OS, please, consider one of the following:

* [General info](https://github.com/tesseract-one/)
* [Tesseract for iOS (Swift)](https://github.com/tesseract-one/Tesseract.swift)
* [Tesseract shared Core (Rust)](https://github.com/tesseract-one/Tesseract.rs)

# Getting started

Tesseract provides two sents of APIs, one for a dApp that wants to connect to the wallets and one for the wallets that want to serve the dApps. Everything below will be split into sections for dApps and Wallets whenever appropriate.

If you'd like to see examples of Tesseract integration, please, check:

* [dev-wallet.kotlin](https://github.com/tesseract-one/dev-wallet.kotlin) - for wallets
* polkachat.kotlin - for dApps, TBD

# Prerequisites

Add the following repo to your `settings.gradle`:

```groovy
maven {
    url = uri("https://maven.tesseract.one/Tesseract.android")
}
```

# For Wallets

Through Tesseract, wallets serve the dApps by providing services accessible from the outside. A service implementation is responsible for understanding requests, providing the user with confirmation UI and replying back.

## Service example

Here is an example of a service implementing support for Substrate protocol from [dev-wallet.kotlin](https://github.com/tesseract-one/dev-wallet.kotlin).

Implementing the service a single time will make it available to all the transport protocols Tesseract covers. Currently we provide only the IPC protocol designed to connect native dApps and Wallets, but more will come soon.

```kotlin
class WalletSubstrateService(private val application: Application, private val settings: KeySettingsProvider): SubstrateService {
    override suspend fun getAccount(type: AccountType): GetAccountResponse {
        val kp = KeyPair.fromMnemonic(settings.load().mnemonic)
        val address = kp.address()

        val accountRequest = SubstrateAccount(type.name, "", address)
        val allow = application.requestUserConfirmation(accountRequest)

        return if(allow) {
            GetAccountResponse(kp.publicKey.toByteArray(), "")
        } else {
            throw UserCancelledException()
        }
    }

    override suspend fun signTransaction(
        accountType: AccountType,
        accountPath: String,
        extrinsicData: ByteArray,
        extrinsicMetadata: ByteArray,
        extrinsicTypes: ByteArray
    ): ByteArray {
        // Parse and show transaction is ommited for the sake of saving space as it's not relevant for Tesseract APIs demonstration

        return if(application.requestUserConfirmation(signRequest)) {
            transaction.sign(kp.secretKey).toByteArray()
        } else {
            throw UserCancelledException()
        }
    }
}
```

## Launching Tesseract instance

```kotlin
val testService = WalletTestService(this, testSettingsProvider())

val substrateService = WalletSubstrateService(this, keySettingsProvider())

tesseract = Tesseract
    .default()
    .service(testService)
    .service(substrateService)
```

It's simple as that. Tesseract will be serving the dApp requests to the services you implemented as long as you keep its reference alive.

## Dependencies

Add the following dependencies:

```groovy
implementation 'one.tesseract:base:0.5.4'
implementation 'one.tesseract:common:0.5.4'
implementation 'one.tesseract:service:0.5.4'
```

## Manifest

To enable the IPC communication, also add the following activity to your `AndroidManifest.xml`. List the mime types for all blockchain prorocols you intend to support.

```xml
<activity
    android:name="one.tesseract.service.transport.ipc.TesseractActivity" android:exported="true">
    <intent-filter>
        <action android:name="one.tesseract.CALL" />

        <category android:name="android.intent.category.DEFAULT" />

        <data android:mimeType="tesseract/test" />
        <data android:mimeType="tesseract/substrate-v1" />
    </intent-filter>
</activity>
```

## Transports

By default Tesseract starts with all the transports enabled. However if you'd like to have a finegrained control over the transports you want to enable, here is how you can do it:

```kotlin
Tesseract()
    .transport(IPCTransport()) //add any transports like this - can be called multiple times
    .service(testService)
    .service(substrateService)
```

To get Tesseract working in a wallet one does not need more: just implement the service and launch the Tesseract instance. However for there also are Kotlin APIs to implement Transports (more ways to communicate with the dApps).

Full documentation on Transports development can be found here: [Transports How To](./TRANSPORTS.MD)

## More

Just in case, you'd like to use Tesseract on android via Rust APIs. It's also possible. Consider checking one of the following:

* [RUST INTEGRATION](./RUST.MD)
* [Developer Wallet in Rust](https://github.com/tesseract-one/dev-wallet)

Also, we have a small library that might help showing the UIs to the user from the service without being aware of the current activity (which is quite a pain without it). Here is where you can check how to use it (sorry, not docs yet, but good examples ;\) ):

* [Source code](./java/detached-activity/)
* [Test wallet example](./examples/native/wallet/)
* [dev-wallet.kotlin](https://github.com/tesseract-one/dev-wallet.kotlin)

See how we request the user confirmation there.

# dApps

We are currently working on this. Soon be here.

# Roadmap

* [x] v0.1 - IPC transport for Android - connect dApp/Wallet on the same device
* [x] v0.2 - demo dApp and Wallet
* [x] v0.3 - Susbtrate protocol support
* [x] v0.4 - [dev-wallet.kotlin](https://github.com/tesseract-one/dev-wallet.kotlin) test implementation
* [x] v0.5 - first Kotlin libraries release version
* [ ] v1.0 - support of everything mobile dApps need

# License

Tesseract.android can be used, distributed and modified under [the Apache 2.0 license](LICENSE).

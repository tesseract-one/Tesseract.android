<p align="left">
	<a href="http://tesseract.one/">
		<img alt="Tesseract" src ="./.github/logo-horiz.svg" height=128/>
	</a>
</p>

# Tesseract on Android for Wallet developers

Integration of Tesseract into your wallet allows any dApp to request your wallet to sign a transaction. Currently Tesseract enables native dApps to integrate with wallets through IPC, which from the user perspective is just a modal screen from the wallet.

## Installation

### Maven Repo

Add the following repo to your `settings.gradle`:

```groovy
maven {
    url = uri("https://maven.tesseract.one/Tesseract.android")
}
```

### Dependencies

Add the following dependencies:

```groovy
implementation 'one.tesseract:base:0.5.4'
implementation 'one.tesseract:common:0.5.4'
implementation 'one.tesseract:service:0.5.4'
```

### Manifest

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

## Initializing

Here is a typical Tesseract initialization snippet (the example is taken from [dev-wallet.kotlin](https://github.com/tesseract-one/dev-wallet.kotlin)):

```kotlin
//instantiate your services

val testService = WalletTestService(this, testSettingsProvider())

val substrateService = WalletSubstrateService(this, keySettingsProvider())

//instantiate Tesseract and keep the instance alive for as long as the wallet app itself lives
tesseract = Tesseract
    .default() //default configuration
    .service(testService) //let Tesseract know about Test service
    .service(substrateService) //let Tesseract know about Substrate service
```

## Services

Through Tesseract, wallets serve the dApps by providing services accessible from the outside. A service implementation is responsible for understanding requests, providing the user with confirmation UI and replying back.

To make Tesseract work in your wallet, you need to describe, how exactly the wallet wants to react when a dApp needs something.

In Tesseract this is done via **services**. One service per blockchain protocol. The way the wallet signs transactions i.e. for Subsrate and for Ethereum is very different, thus every service has its own API to implement.

Let's take a look at the `SubstrateService` implementation example from [dev-wallet.kotlin](https://github.com/tesseract-one/dev-wallet.kotlin).:

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

The above code demonstrates that the wallet needs to know how to react to two scenarious:

1. The dApp asks for the account (`getAccount` method)
2. The dApp wants to sign the transaction (`signTransaction` method)

In both methods the wallet is responsible to present the user with the relevant UI and reply with a response in case the user agrees to proced. Otherwise just through a `UserCancelledException`. Or any other exception if, in example, the request data is malformed.

### Service flavors

Pay attention that the methods in the example above are of `suspend` type, which means that they are asynchronous by nature and you can use the **Kotlin Coroutines** without any hastle. `suspend` services are not the only option though. In case you don't want to use coroutines for some reason, Tesseract also provides equivalent interfaces that work with plain java `Futures`. All the services interfaces are located in `one.tesseract.common.protocol` package, and are split into two groups:

* `java` - java futures based interfaces
* `kotlin` - kotlin coroutines based interfaces

The main difference between the two are that the `java` counterparts are not `suspend` and require the implementer to return `CompletionStage<Response>` instead of `Response`. Don't use the blocking APIs in neither, though.

## Transports

By default Tesseract starts with all the transports enabled. However if you'd like to have a finegrained control over the transports you want to enable, here is how you can do it:

```kotlin
Tesseract()
    .transport(IPCTransport()) //add any transports like this - can be called multiple times
    .service(testService)
    .service(substrateService)
```

Full documentation on Transports development can be found here: [Transports How To](./TRANSPORTS.MD)

## Presenting UI from service

From our experince it might be quite painful to present the user with an activity and get a response from it, especially when it's NOT done from another activity.

For that we have crated a utility library: [detached-activity](./java/detached-activity/).

It should make presenting the user with confirmation dialogs from the services pretty easy. At least it does the job for us.

## Conclusion

We tried our best to present an API as easy for the wallet developer as we could and handled all the edge cases we know of inside the library. At least we improved it to the point that it satisfied us while building the [dev-wallet.kotlin](https://github.com/tesseract-one/dev-wallet.kotlin).

If you have any suggestions, please, create an issue or submit a PR.

Thanks!

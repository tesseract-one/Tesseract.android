<p align="left">
	<a href="http://tesseract.one/">
		<img alt="Tesseract" src ="./.github/logo-horiz.svg" height=128/>
	</a>
</p>

# Tesseract on Android for dApp developers

Integration of Tesseract into your dApp allows to connect to any wallet, that supports Tesseract protocol, to sign a transaction. Currently Tesseract enables native dApps to integrate with wallets through IPC, which from the user perspective is just a modal screen from the wallet.

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
implementation 'one.tesseract:base:0.5.6'
implementation 'one.tesseract:common:0.5.6'
implementation 'one.tesseract:client:0.5.6'
```

### Manifest

To enable the IPC communication, also add the following activity to your `AndroidManifest.xml`.

```xml
<activity android:name="one.tesseract.client.transport.ipc.TesseractActivity" android:exported="false" android:label="Tesseract"></activity>
```

Also, we need to allows the dApp to make the calls to the wallets:

```xml
<queries>
    <intent>
        <action android:name="one.tesseract.CALL" />
        <category android:name="android.intent.category.DEFAULT" />
        <data android:mimeType="tesseract/*" />
    </intent>
</queries>
```

## Initializing

Here is a typical Tesseract initialization snippet (the example is taken from [polkachat.kotlin](https://github.com/tesseract-one/polkachat.kotlin)):

```kotlin
class Core(private val application: Application) {
    private val tesseract: Tesseract = Tesseract.default(application)
    private val service: SubstrateService = tesseract.service(SubstrateService::class)

//your code here
}
```

## Services

Through Tesseract, wallets serve the dApps by providing services accessible from the outside. A dApp can connect to such a service and query the wallet for an account address, signature, etc.

To make Tesseract work in your dApp, the only thing you need is to initialize a Tesseract and service instances (like it was shown above) and start calling service methods.

Each service represents one blockchain protocol. This way, with the `SubstrateService` you can query wallets that support substrate.

Calling the service methods looks as simple as following code:

```kotlin
//querying the wallet for ther users account address
val account = service.getAccount(AccountType.Sr25519)
```

or

```kotlin
val signature = service.signTransaction(account.type, account.path, payload, metadata, types)
//account.type is the same you passed when requesting the address
//account.path was returned by the wallet in getAccount
//payload, metadata, types - this is your dApp's transaction and network data (see polkachat.kotlin for an example)
```

Tesseract protocol will handle all the transitions to the wallet UI and the UI presented is provided by the wallet itself. You don't need to do anything here.

If the request fails, the mothods can throw an exception with an error description. To indicate that the user cancelled the request, a `UserCancelledException` will be thrown (so make sure you handle it properly).

### Service flavors

The methods in the example above are of `suspend` type, which means that they are asynchronous by nature and you can use the **Kotlin Coroutines** without any hastle. `suspend` services are not the only option though. In case you don't want to use coroutines for some reason, Tesseract also provides equivalent interfaces that work with plain java `Futures`. All the services interfaces are located in `one.tesseract.common.protocol` package, and are split into two groups:

* `java` - java futures based interfaces
* `kotlin` - kotlin coroutines based interfaces

The main difference between the two are that the `java` counterparts are not `suspend` and return `CompletionStage<Response>` instead of `Response`. Neither flavour is blocking and is fully asynchronous.

## Transports and customization

By default Tesseract starts with all the transports enabled. However if you'd like to have a finegrained control over the transports you want to enable, here is how you can do it:

```kotlin
//implement delegate to provide the transport selection logic
class MyDelegate: Delegate {
    override suspend fun selectTransport(transports: Map<String, Status>): String? {
        //replace with your actual transport selection logic
        //just returning a transport without checking a status
        //is a bad practice. Return null to select none (the
        //call will be canceled in this case)
        return transports.keys.first()
    }
}

//change Tesseract initialization to this
Tesseract(delegate = MyDelegate())
    .transport(IPCTransport(application)) //add any transports like this - can be called multiple times
```

also add the dependency to transpprt definitions:

```groovy
implementation "one.tesseract:client-transport:0.5.6"
implementation "one.tesseract:client-transport-ipc:0.5.6"
```

The above needs a little explanation. First of all, the notion of delegate. With the custom delegate, a dApp developer can provide custom transport selection logic (i.e. present the user with the transport selection UI) in case there are multiple transports enabled.

Delegate is also available in two flavors: `java` and `kotlin` providing `futures` and `suspended` approaches to asynchronous calls respectively.

As many transports as needed can be registered in the Tesseract instance.

Full documentation on Transports development can be found here: [Transports How To](./TRANSPORTS.MD)

## Conclusion

We tried our best to present an API as easy for the dApp developer as we could and handled all the edge cases we know of inside the library. At least we improved it to the point that it satisfied us while building the [polkachat.kotlin](https://github.com/tesseract-one/polkachat.kotlin).

If you have any suggestions, please, create an issue or submit a PR.

Thanks!

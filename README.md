<p align="center">
	<a href="http://tesseract.one/">
		<img alt="Tesseract" src ="./.github/logo-vert.svg" height=256/>
	</a>
</p>

# Tesseract Android

**Tesseract Android** provides Java/Kotlin APIs for [Tesseract](https://github.com/tesseract-one/), a dApp-Wallet bridge designed to make dApp/wallet communication on mobile devices simple and natural without compromising decentralization and security

If you are looking for Tesseract docs for another language/OS, please, consider one of the following:

* [General info](https://github.com/tesseract-one/)
* [Tesseract Swift (iOS)](https://github.com/tesseract-one/Tesseract.swift)
* [Tesseract Rust](https://github.com/tesseract-one/Tesseract.rs)

## Getting started

Tesseract provides two sets of APIs, one for a dApp that wants to connect to the wallets and one for the wallets that want to serve the dApps.

Here is how a typical Tesseract workflow looks like:

<table>
<tr>
<th> dApp </th>
<th> Wallet </th>
</tr>
<tr>
<td>

```kotlin
//initialize Tesseract
val tesseract = Tesseract.default()

//indicate what blockchain are we gonna use
val substrateService = tesseract.service(SubstrateService.class)

//at this point Tesseract connects to the
//wallet and the wallet presents the user
//with its screen, asking if the user
//wants to share their public key to a dApp
val account = substrateService.getAccount(AccountType.Sr25519)
```

</td>
<td>

```kotlin
//Inside the Wallet Tesseract serves requests
//from the dApps as long as the reference is kept alive
//save it somewhere in the Application instance
val tesseract = Tesseract
    .default() //start with default configuration
    .service(MySubstrateService())
//MySubstrateService instance methods
//will be called when a dApp asks for something
```

</td>
</tr>
</table>

## Details

Because using Tesseract in Tesseract in a dApp and in a wallet is very different by nature (essentially communicating as a client and a service), the detailed documentation is split into two documents:

* [Tesseract for dApp developers](./DAPP.MD)
* [Tesseract for Wallet developers](./WALLET.MD)

## Examples

If you'd like to see examples of Tesseract integration, please, check:

* [dev-wallet.kotlin](https://github.com/tesseract-one/dev-wallet.kotlin) - for wallets
* polkachat.kotlin - for dApps, TBD

## More

Just in case, you'd like to use Tesseract on android via Rust APIs. It's also possible. Consider checking one of the following:

* [Using Tesseract on Android in Rust](./RUST.MD)
* [Developer Wallet in Rust](https://github.com/tesseract-one/dev-wallet)

## Roadmap

* [x] v0.1 - IPC transport for Android - connect dApp/Wallet on the same device
* [x] v0.2 - demo dApp and Wallet
* [x] v0.3 - Susbtrate protocol support
* [x] v0.4 - [dev-wallet.kotlin](https://github.com/tesseract-one/dev-wallet.kotlin) test implementation
* [x] v0.5 - first Kotlin libraries release version
* [ ] v1.0 - support of everything mobile dApps need

## License

Tesseract.android can be used, distributed and modified under [the Apache 2.0 license](LICENSE).

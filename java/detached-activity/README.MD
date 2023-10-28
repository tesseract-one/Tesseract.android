# Detached Activity

Detached Activity is a small library that helps to launch activities and accepting the results from them without knowing anything of the currently running activity.

## Installation

```groovy
implementation "one.tesseract:detached-activity:0.2.0"
```

## Usage

Initialize the detached activity launcher on application start:

```kotlin
val launcher = Launcher(this) //'this' is and instance of Application
```

You can start an activity by calling one of the methods of `Launcher` that start with `startDetachedActivityForResult<Result>`. They are all the same, just return the result in different form. The arguments to pass are the `activityClass` and `extras` (which you can later retrieve from the activity's `onCreate`).

To end such a (*detached*) activity, call `finishDetachedActivity(RESULT_OK, RESULT_TYPE)` or `finishDetachedActivity(RESULT_CANCEL, RESULT_TYPE)`, depending on if the user tapped **OK** or **Cancel**.

Here is an example of such an activity that can return a `Boolean` result:

```kotlin
class SignActivity : ComponentActivity() {
    companion object {
        suspend fun requestUserConfirmation(launcher: Launcher, transaction: String): Boolean {
            val extras = Bundle()
            extras.putString("transaction", transaction)

            return launcher
                .startDetachedActivityForResult<Boolean>(
                    SignActivity::class.java,
                    extras)
                .second
        }
    }
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val extras = intent.extras ?: throw RuntimeException("No Extras :(")
        val transaction = extras.getString("transaction") ?: throw RuntimeException("No Transaction :(")

        setContent {
            Text(
                text = "Want to sign this transaction?\nTransaction: $transaction!",
            )

            Button(onClick = {
                finishDetachedActivity(RESULT_OK, true)
            }) {
                Text(text = "OK")
            }
            Button(onClick = {
                finishDetachedActivity(RESULT_CANCELED, false)
            }) {
                Text(text = "Cancel")
            }
        }
    }
}
```

## Examples

In [this](../../examples/native/wallet) wallet example, we use it to launch the `SignActivity`. Hopefully, some copypasting from there is good enough.

Also, here is a more complex example that uses `Parcelable` objects to pass to the activity here: [dev-wallet.kotlin](https://github.com/tesseract-one/dev-wallet.kotlin).

## License

Tesseract.android can be used, distributed and modified under [the Apache 2.0 license](../../LICENSE).

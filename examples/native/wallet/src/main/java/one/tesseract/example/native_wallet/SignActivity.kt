package one.tesseract.example.native_wallet

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview

import one.tesseract.activity.detached.Launcher
import one.tesseract.activity.detached.finishDetachedActivity

import one.tesseract.example.native_wallet.ui.theme.TesseractAndroidTheme

class SignActivity : ComponentActivity() {
    companion object {
        suspend fun requestUserConfirmation(launcher: Launcher, transaction: String): Boolean {
            return launcher
                .startDetachedActivityForResult<Boolean>(
                    SignActivity::class.java,
                    Bundle().withTransaction(transaction))
                .second
        }
    }
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val extras = intent.extras ?: throw RuntimeException("No Extras :(")
        val transaction = extras.transaction

        setContent {
            TesseractAndroidTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    Column {
                        Greeting2("Transaction: $transaction")
                        Row {
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
            }
        }
    }
}

@Composable
fun Greeting2(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview2() {
    TesseractAndroidTheme {
        Greeting2("Android")
    }
}
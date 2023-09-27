package one.tesseract.wallet

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

import one.tesseract.ipc.activity.free.Launcher
import one.tesseract.ipc.activity.free.finishFreeActivity

import one.tesseract.wallet.ui.theme.TesseractAndroidTheme

class SignActivity : ComponentActivity() {
    companion object {
        const val TRANSACTION = "transaction"

        suspend fun requestUserConfirmation(launcher: Launcher, transaction: String): Boolean {
            val bundle = Bundle()
            bundle.putString(TRANSACTION, transaction)
            return launcher.startFreeActivityForResult<Boolean>(SignActivity::class.java, bundle).second
        }
    }
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val extras = intent.extras ?: throw RuntimeException("No Extras :(")
        val transaction = extras.getString(TRANSACTION) ?: throw RuntimeException("No TX")

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
                                finishFreeActivity(RESULT_OK, true)
                            }) {
                                Text(text = "OK")
                            }
                            Button(onClick = {
                                finishFreeActivity(RESULT_CANCELED, false)
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
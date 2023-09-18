package one.tesseract.wallet

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import one.tesseract.service.Tesseract
import one.tesseract.service.protocol.TestService
import one.tesseract.service.transport.IPCTransport
import one.tesseract.wallet.ui.theme.TesseractAndroidTheme
import java.util.concurrent.CompletableFuture
import java.util.concurrent.CompletionStage
import java.util.concurrent.Future

class WalletTestService: TestService {
    override fun signTransaction(transaction: String): CompletionStage<String> = CompletableFuture.completedFuture(transaction + "_signed")
}

class MainActivity(@Suppress("unused") private var tesseract: Tesseract? = null) : ComponentActivity() {
    init {
        val tes = Tesseract()
        tes.addTransport(IPCTransport())
        tes.addService(WalletTestService())
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        setContent {
            TesseractAndroidTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    Greeting("Android2")
                }
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    TesseractAndroidTheme {
        Greeting("Android")
    }
}
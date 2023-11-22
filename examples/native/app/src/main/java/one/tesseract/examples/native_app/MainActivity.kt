package one.tesseract.examples.native_app

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import kotlinx.coroutines.launch
import one.tesseract.client.Tesseract
import one.tesseract.examples.native_app.ui.theme.TesseractAndroidTheme
import one.tesseract.protocol.kotlin.TestService

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        val tesseract = Tesseract.default(this.application)
        val testService = tesseract.service(TestService::class)

        super.onCreate(savedInstanceState)
        setContent {
            TesseractAndroidTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    val coroutineScope = rememberCoroutineScope()

                    Button(onClick = {
                        coroutineScope.launch {
                            val result = testService.signTransaction("my transaction")
                            Log.d("DAPP", "!!!!It arrived!!!: $result")
                        }
                    }) {
                        Text(
                            text = "Sign"
                        )
                    }
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
package one.tesseract.examples.native_app

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import one.tesseract.client.Tesseract
import one.tesseract.examples.native_app.ui.theme.TesseractAndroidTheme
import one.tesseract.exception.UserCancelledException
import one.tesseract.protocol.kotlin.TestService
import java.lang.RuntimeException

class MainActivity : ComponentActivity() {
    @OptIn(ExperimentalMaterial3Api::class)
    override fun onCreate(savedInstanceState: Bundle?) {
        val tesseract = Tesseract.default(delegate = AppDelegate(), application = this.application)
        val testService = tesseract.service(TestService::class)

        super.onCreate(savedInstanceState)
        setContent {
            TesseractAndroidTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background
                ) {
                    val coroutineScope = rememberCoroutineScope()
                    var transaction by remember { mutableStateOf("test transaction") }
                    var result by remember { mutableStateOf("") }

                    Column(
                        modifier = Modifier.fillMaxSize(),
                        verticalArrangement = Arrangement.Center,
                        horizontalAlignment = Alignment.CenterHorizontally
                    ) {
                        Row(
                            modifier = Modifier.padding(horizontal = 24.dp),
                            horizontalArrangement = Arrangement.SpaceBetween
                        ) {
                            TextField(
                                value = transaction,
                                onValueChange = { transaction = it },
                                modifier = Modifier
                                    .defaultMinSize(minWidth = 1.dp)
                                    .fillMaxWidth(0.7f)
                            )
                            Spacer(modifier = Modifier.defaultMinSize(minWidth = 16.dp))
                            Button(
                                modifier = Modifier.align(Alignment.CenterVertically),
                                onClick = {
                                    coroutineScope.launch {
                                        try {
                                            result = testService.signTransaction(transaction)
                                        } catch (_: UserCancelledException) {
                                            result = "You cancelled... OK"
                                        } catch (e: RuntimeException) {
                                            result = "An error occurred: ${e.message}"
                                            e.printStackTrace()
                                        }
                                    }
                                }
                            ) {
                                Text(text = "Sign")
                            }
                        }
                        Text(
                            text = result,
                            modifier = Modifier
                                .padding(top = 16.dp)
                                .align(Alignment.CenterHorizontally)
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
        text = "Hello $name!", modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    TesseractAndroidTheme {
        Greeting("Android")
    }
}
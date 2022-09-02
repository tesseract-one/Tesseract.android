package one.tesseract.example.app

import android.os.Build
import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import androidx.annotation.RequiresApi
import androidx.appcompat.app.AppCompatActivity
import java.util.concurrent.CompletionStage

class MainActivity : AppCompatActivity() {
    private val core: RustCore
        get() = (this.application as Application).rustCore

    private fun signTransaction(): CompletionStage<String> {
        val transactionText = findViewById<EditText>(R.id.editTextTransaction)

        val transaction = transactionText.text.toString()
        return core.sign(transaction)
    }


    @RequiresApi(Build.VERSION_CODES.N)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val buttonSign = findViewById<Button>(R.id.buttonSign)
        val buttonCheckPools = findViewById<Button>(R.id.buttonCheckPools)
        val signatureText = findViewById<TextView>(R.id.textViewSignature)

        buttonSign.setOnClickListener {
            signTransaction().whenComplete { result, error ->
                signatureText.text = result
            }
        }

        buttonCheckPools.setOnClickListener {
            core.execute(signTransaction())
        }
    }
}
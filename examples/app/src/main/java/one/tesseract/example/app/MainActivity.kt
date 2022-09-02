package one.tesseract.example.app

import android.os.Build
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import androidx.annotation.RequiresApi

class MainActivity : AppCompatActivity() {
    @RequiresApi(Build.VERSION_CODES.N)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val application = this.application as Application
        val core = application.rustCore

        val buttonSign = findViewById<Button>(R.id.buttonSign)
        val transactionText = findViewById<EditText>(R.id.editTextTransaction)
        val signatureText = findViewById<TextView>(R.id.textViewSignature)

        buttonSign.setOnClickListener {
            val transaction = transactionText.text.toString()
            core.sign(transaction)
                .whenComplete { result, error ->
                    signatureText.text = result
                }
        }
    }
}
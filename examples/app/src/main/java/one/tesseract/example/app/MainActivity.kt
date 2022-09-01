package one.tesseract.example.app

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import android.widget.TextView

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val application = this.application as Application
        val core = application.rustCore

        val buttonSign = findViewById<Button>(R.id.buttonSign)
        val transactionText = findViewById<EditText>(R.id.editTextTransaction)
        val signatureText = findViewById<TextView>(R.id.textViewSignature)

        buttonSign.setOnClickListener {
            val transaction = transactionText.text
            core.makeTransaction()
        }
    }
}
package one.tesseract.example.wallet

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.Button
import android.widget.TextView
import one.tesseract.ipc.activity.free.Launcher

import one.tesseract.ipc.activity.free.finishFreeActivity
import java.util.concurrent.CompletionStage

class SignActivity : AppCompatActivity() {
    companion object {
        const val TRANSACTION = "transaction"

        fun requestUserConfirmation(launcher: Launcher, transaction: String): CompletionStage<Boolean> {
            val bundle = Bundle()
            bundle.putString(TRANSACTION, transaction)
            return launcher.startFreeActivityForResult<Boolean>(SignActivity::class.java, bundle).thenApply {
                it.second
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_sign)

        val extras = intent.extras ?: throw RuntimeException("No Extras :(")
        val transaction = extras.getString(TRANSACTION) ?: throw RuntimeException("No TX")

        val buttonSign = findViewById<Button>(R.id.buttonSign)
        val buttonCancel = findViewById<Button>(R.id.buttonCancel)
        val textViewTransaction = findViewById<TextView>(R.id.textViewTransaction)

        textViewTransaction.text = transaction

        buttonSign.setOnClickListener {
            this.finishFreeActivity(RESULT_OK, true)
        }

        buttonCancel.setOnClickListener {
            this.finishFreeActivity(RESULT_CANCELED, false)
        }
    }
}
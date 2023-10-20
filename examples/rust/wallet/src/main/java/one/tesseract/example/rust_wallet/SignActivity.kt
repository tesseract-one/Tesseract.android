package one.tesseract.example.rust_wallet

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.Button
import android.widget.TextView

import java.util.concurrent.CompletionStage

import one.tesseract.activity.detached.Launcher
import one.tesseract.activity.detached.finishDetachedActivity
import one.tesseract.activity.detached.getExtras

class SignActivity : AppCompatActivity() {
    companion object {
        fun requestUserConfirmation(launcher: Launcher, transaction: String): CompletionStage<Boolean> {
            return launcher.startDetachedActivityForResultFuture<Boolean>(
                SignActivity::class.java,
                Bundle().withTransaction(transaction)).
            thenApply {
                it.second
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_sign)

        val extras = getExtras() ?: throw RuntimeException("No Extras :(")
        val transaction = extras.transaction

        val buttonSign = findViewById<Button>(R.id.buttonSign)
        val buttonCancel = findViewById<Button>(R.id.buttonCancel)
        val textViewTransaction = findViewById<TextView>(R.id.textViewTransaction)

        textViewTransaction.text = transaction

        buttonSign.setOnClickListener {
            this.finishDetachedActivity(RESULT_OK, true)
        }

        buttonCancel.setOnClickListener {
            this.finishDetachedActivity(RESULT_CANCELED, false)
        }
    }
}
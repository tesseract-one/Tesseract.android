package one.tesseract.service.transport.ipc

import android.app.Activity
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Bundle

import one.tesseract.transport.ipc.ResponseBundle
import one.tesseract.transport.ipc.id
import one.tesseract.transport.ipc.tx

class TesseractActivity : Activity() {
    companion object {
        const val DEFAULT_CHANNEL: String = "default"
    }

    private fun channelId(): String {
        val ai = this.packageManager.getActivityInfo(
            this.componentName,
            PackageManager.GET_ACTIVITIES.or(PackageManager.GET_META_DATA)
        )

        val metaData: Bundle? = ai.metaData
        return if (metaData == null) {
            DEFAULT_CHANNEL
        } else {
            metaData.getString("channel") ?: DEFAULT_CHANNEL
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val extras = intent.extras ?: throw RuntimeException("No Extras :(")
        val id = extras.id ?: throw RuntimeException("No ID")
        val data = extras.tx ?: throw RuntimeException("No TX")

        val channelId = this.channelId()

        val response =
            Channel.send(channelId, data)
                ?: throw RuntimeException("No channel '$channelId' found")

        response.whenComplete { r, _ ->
            val bundle = ResponseBundle(id, r)

            val intent = Intent()
            intent.action = "one.tesseract.REPLY"

            intent.putExtras(bundle)

            setResult(RESULT_OK, intent)

            finish()
        }
    }
}
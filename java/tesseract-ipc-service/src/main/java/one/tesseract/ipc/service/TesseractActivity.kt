package one.tesseract.ipc.service

import android.app.Activity
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.util.Log
import androidx.annotation.RequiresApi

import one.tesseract.ipc.*;

class TesseractActivity : Activity() {
    companion object {
        const val DEFAULT_CHANNEL: String = "channel"
    }

    private fun channelId(): String {
        val ai = this.packageManager.getActivityInfo(
            this.getComponentName(),
            PackageManager.GET_ACTIVITIES.or(PackageManager.GET_META_DATA)
        )

        val metaData: Bundle = ai.metaData
        return if (metaData == null) {
            DEFAULT_CHANNEL
        } else {
            metaData.getString("channel") ?: DEFAULT_CHANNEL
        }
    }

    @RequiresApi(Build.VERSION_CODES.N)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val extras = intent.extras ?: throw RuntimeException("No Extras :(")
        val id = extras.id ?: throw RuntimeException("No ID")
        val data = extras.tx ?: throw RuntimeException("No TX")

        val channelId = this.channelId()

//        val channel =
//            Channel.send(channelId, data)
//                ?: throw RuntimeException("No channel '$channelId' found")

//        channel.whenComplete { data, error ->
//            run {
        val intent = Intent()
        intent.action = "one.tesseract.REPLY"

        Log.v("TEST", "before $data")

        val bundle = {
            val data =
                "json{\"id\":1,\"response\":{\"status\":\"ok\",\"signed\":\"testTransaction_signed!\"}}".toByteArray();
            val bundle = Bundle(2)
            bundle.rx = data
            bundle.id = id
            bundle
        }()

            intent.putExtras(bundle)

            setResult(Activity.RESULT_OK, intent)

            finish()
//            }
//        }
        }
    }
//}
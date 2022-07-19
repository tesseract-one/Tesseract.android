//===------------ TesseractActivity.kt --------------------------------------------===//
//  Copyright 2022, Tesseract Systems, Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//===----------------------------------------------------------------------===//

package one.tesseract.ipc

import android.app.Activity
import android.content.Intent
import android.os.Build
import android.os.Bundle
import android.util.Log
import androidx.annotation.RequiresApi
import java.util.*


class TesseractActivity: Activity() {
    private companion object Randomizer {
        val _random: Random = Random(System.currentTimeMillis())

        fun random(): Int {
            synchronized(_random) {
                var res = -1
                while (res < 1) {
                    res = _random.nextInt()
                }
                return res
            }
        }
    }

    val reqCode:Int = random()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val data = getIntent().getExtras()
            ?: throw RuntimeException("Invalid API usage of Tesseract Activity")

        val intent = Intent()
        intent.action = "one.tesseract.CALL"
        intent.putExtras(data)
        //intent.addCategory("")
        //intent.ca
        //intent.type = "text/plain"

//        object : CountDownTimer(1000, 1) {
//            override fun onFinish() {
                startActivityForResult(intent, reqCode)
//            }
//            override fun onTick(millisUntilFinished: Long) {}
//        }.start()



        //finish()
    }

    @RequiresApi(Build.VERSION_CODES.N)
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        if(requestCode != reqCode) {
            Log.d("ERROR", "Malfunctioning wallet: wrong requestCode")
        } else {
            val response = data?.extras
            if(response == null) {
                val response = getIntent().getExtras()?.toEmptyResponse()
                    ?: throw RuntimeException("Invalid API usage of Tesseract Activity")
                //Log.d("ERROR", "The wallet returned no data")
                TransceiverRegistry.resolve(Pair(resultCode, response))
            } else {
                TransceiverRegistry.resolve(Pair(resultCode, response))
            }
        }

        super.onActivityResult(requestCode, resultCode, data)

        //setResult(Activity.RESULT_OK)
        finish()



    }

//    override fun onStop() {
//        super.onStop()
//        finish()
//    }
}
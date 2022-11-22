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

package one.tesseract.ipc.client

import android.app.Activity
import android.content.Intent
import android.os.Build
import android.os.Bundle
import android.util.Log
import androidx.annotation.RequiresApi
import java.util.*

import one.tesseract.ipc.*


class TesseractActivity : Activity() {
    private companion object Randomizer {
        val random: Random = Random(System.currentTimeMillis())

        fun random(): Int {
            synchronized(random) {
                var res = -1
                while (res < 1) {
                    res = random.nextInt()
                }
                return res
            }
        }
    }

    private val reqCode: Int = random()
    private var response: Pair<Int, Bundle>? = null

    private fun emptyResponse(): Bundle = intent.extras?.toEmptyResponse()
        ?: throw RuntimeException("Invalid API usage of Tesseract Activity")

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val intent = intent.convertToCall()

        startActivityForResult(intent, reqCode)
    }


    @RequiresApi(Build.VERSION_CODES.N)
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        if (requestCode != reqCode) {
            Log.d("ERROR", "Malfunctioning wallet: wrong requestCode")
        } else {
            val extras = data?.extras
            this.response = if (extras == null) {
                //Log.d("ERROR", "The wallet returned no data")
                Pair(resultCode, emptyResponse())
            } else {
                Pair(resultCode, extras)
            }
        }

        super.onActivityResult(requestCode, resultCode, data)

        finish()
    }

    @RequiresApi(Build.VERSION_CODES.N)
    override fun onDestroy() {
        val response = this.response ?: Pair(
            500,
            emptyResponse()
        )//TODO: take in account code 500 when parsing. Good for now though
        TransceiverRegistry.resolve(response)

        super.onDestroy()
    }
}
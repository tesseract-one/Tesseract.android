//===------------ Application.kt --------------------------------------------===//
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

package one.tesseract.example.app

import android.app.AlertDialog
import one.tesseract.ipc.activity.ActivityMonitor

class Application: android.app.Application() {
    lateinit var rustCore: RustCore
    private lateinit var monitor: ActivityMonitor

    @Suppress("unused") //because it's used from rust
    fun showAlert(message: String) {
        AlertDialog.Builder(monitor.activity).setMessage(message).setPositiveButton(android.R.string.ok
        ) { dialog, _ ->
            dialog.dismiss()
        }.create().show()
    }

    override fun onCreate() {
        monitor = ActivityMonitor(this)
        rustCore = RustCore(this)

        super.onCreate()
    }
}
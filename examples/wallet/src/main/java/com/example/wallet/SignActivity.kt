//===------------ SignActivity.kt --------------------------------------------===//
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

package com.example.wallet

import android.app.Activity
import android.content.Intent
import android.os.Bundle
import android.util.Log
import com.google.android.material.snackbar.Snackbar
import androidx.appcompat.app.AppCompatActivity
import androidx.navigation.findNavController
import androidx.navigation.ui.AppBarConfiguration
import androidx.navigation.ui.navigateUp
import androidx.navigation.ui.setupActionBarWithNavController
import com.example.wallet.databinding.ActivitySignBinding

class SignActivity : AppCompatActivity() {

    private lateinit var appBarConfiguration: AppBarConfiguration
    private lateinit var binding: ActivitySignBinding

    private var id: String? = null
    private var data: ByteArray? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val extras = intent.extras ?: throw RuntimeException("No Extras :(")
        this.id = extras.getString("id")
        this.data = extras.getByteArray("tx")

        //Log.v("REQ", extras.getString("data") ?: throw RuntimeException("No Data :("))

        binding = ActivitySignBinding.inflate(layoutInflater)
        setContentView(binding.root)

        setSupportActionBar(binding.toolbar)

        val navController = findNavController(R.id.nav_host_fragment_content_sign)
        appBarConfiguration = AppBarConfiguration(navController.graph)
        setupActionBarWithNavController(navController, appBarConfiguration)

        binding.fab.setOnClickListener { view ->
//            Snackbar.make(view, "Replace with your own action", Snackbar.LENGTH_LONG)
//                .setAction("Action", null).show()

            val intent = Intent()
            intent.action = "one.tesseract.REPLY"

            Log.v("TEST", "before $data")

            //val test = data?.let { (String(it) + "Signed").toByteArray() }

            val test = "json{\"id\":1,\"response\":{\"status\":\"ok\",\"signed\":\"testTransaction_signed!\"}}".toByteArray();
            //val test = "json{\"id\":1,\"response\":{\"status\":\"error\",\"kind\":\"weird\",\"description\":\"intentional error for test\"}}".toByteArray();

            Log.v("TEST", "after $test")

            val bundle = Bundle(2)
            bundle.putByteArray("rx", test)
            bundle.putString("id", id)

            intent.putExtras(bundle)

            setResult(Activity.RESULT_OK, intent)

            finish()

        }
    }

    override fun onSupportNavigateUp(): Boolean {
        val navController = findNavController(R.id.nav_host_fragment_content_sign)
        return navController.navigateUp(appBarConfiguration)
                || super.onSupportNavigateUp()
    }
}
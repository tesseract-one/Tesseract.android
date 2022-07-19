//===------------ MainActivity.kt --------------------------------------------===//
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

package com.example.clientapp

import android.os.Build
import android.os.Bundle
import android.util.Log
import android.view.Menu
import android.view.MenuItem
import androidx.annotation.RequiresApi
import androidx.appcompat.app.AppCompatActivity
import androidx.navigation.findNavController
import androidx.navigation.ui.AppBarConfiguration
import androidx.navigation.ui.navigateUp
import androidx.navigation.ui.setupActionBarWithNavController
import com.example.clientapp.databinding.ActivityMainBinding


class MainActivity : AppCompatActivity() {


    private external fun helloRust(hi: String): String

    private lateinit var appBarConfiguration: AppBarConfiguration
    private lateinit var binding: ActivityMainBinding

    @RequiresApi(Build.VERSION_CODES.S)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        Log.v("DEBUG", helloRust("heyhey"))

        /*this.application.registerActivityLifecycleCallbacks(
            object : ActivityLifecycleCallbacks {
                override fun onActivityCreated(activity: Activity, savedInstanceState: Bundle?) {
                    Log.v(null, "onActivityCreated")
                }

                override fun onActivityStarted(activity: Activity) {

                    this@MainActivity.currentActivity = activity
                }

                override fun onActivityResumed(activity: Activity) {
                    Log.v(null, "onActivityResumed")

                }

                override fun onActivityPaused(activity: Activity) {
                    Log.v(null, "onActivityPaused")
                }

                override fun onActivityStopped(activity: Activity) {
                    Log.v(null, "onActivityStopped")
                }

                override fun onActivitySaveInstanceState(activity: Activity, outState: Bundle) {
                    Log.v(null, "onActivitySaveInstanceState")
                }

                override fun onActivityDestroyed(activity: Activity) {
                    Log.v(null, "onActivityDestroyed")
                }
            }
        )*/



        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        setSupportActionBar(binding.toolbar)

        val navController = findNavController(R.id.nav_host_fragment_content_main)
        appBarConfiguration = AppBarConfiguration(navController.graph)
        setupActionBarWithNavController(navController, appBarConfiguration)

        binding.fab.setOnClickListener { view ->
            //Snackbar.make(view, "Replace with your own action", Snackbar.LENGTH_LONG)
            //        .setAction("Action", null).show()
//            val intent = Intent()
//            intent.action = "one.tesseract.CALL"
            //intent.addCategory("")
            //intent.ca
            //intent.type = "text/plain"

            //this.applicationContext.startActivity(intent)

//            val context = applicationContext
//
//            //application.startActivity()
//
//            val data = Bundle(2)
//            data.putString("data", "TransactionToSign")
//            data.putString("id", UUID.randomUUID().toString())
//
//            var intent = Intent(context, TesseractActivity::class.java)
//            intent.putExtras(data)
//
//            startActivity(intent)

            Application.rustCore.makeTransaction()
        }
    }

    override fun onCreateOptionsMenu(menu: Menu): Boolean {
        // Inflate the menu; this adds items to the action bar if it is present.
        menuInflater.inflate(R.menu.menu_main, menu)
        return true
    }

    override fun onOptionsItemSelected(item: MenuItem): Boolean {
        // Handle action bar item clicks here. The action bar will
        // automatically handle clicks on the Home/Up button, so long
        // as you specify a parent activity in AndroidManifest.xml.
        return when (item.itemId) {
            R.id.action_settings -> true
            else -> super.onOptionsItemSelected(item)
        }
    }

    override fun onSupportNavigateUp(): Boolean {
        val navController = findNavController(R.id.nav_host_fragment_content_main)
        return navController.navigateUp(appBarConfiguration)
                || super.onSupportNavigateUp()
    }
}
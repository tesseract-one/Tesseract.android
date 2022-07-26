package com.example.wallet

class Application: android.app.Application() {
    companion object {
        lateinit var rustCore: RustCore
    }

    override fun onCreate() {
        super.onCreate()

        Application.rustCore = RustCore(this)
    }
}
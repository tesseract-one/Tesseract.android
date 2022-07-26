package com.example.wallet

class RustCore(public val application: Application) {
    companion object {
        init {
            System.loadLibrary("wallet")
        }
    }
}
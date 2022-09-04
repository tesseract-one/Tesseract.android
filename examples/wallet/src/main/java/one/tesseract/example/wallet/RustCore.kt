package one.tesseract.example.wallet

class RustCore(public val application: Application) {
//    val processor: Processor = object : Processor {
//        override fun process(data: ByteArray): CompletionStage<ByteArray> {
//            return CompletableFuture.completedFuture(
//                "json{\"id\":1,\"response\":{\"status\":\"ok\",\"signed\":\"testTransaction_signed!\"}}".toByteArray()
//            )
//        }
//    }

    init {
        rustInit(this.javaClass.classLoader)
    }

    private external fun rustInit(loader: ClassLoader)

//    val channel: Channel = Channel.create("default") { data ->
//        CompletableFuture.completedFuture(
//            "json{\"id\":1,\"response\":{\"status\":\"ok\",\"signed\":\"testTransaction_signed!\"}}".toByteArray()
//        )
//    }

    companion object {
        init {
            System.loadLibrary("wallet")
        }
    }
}
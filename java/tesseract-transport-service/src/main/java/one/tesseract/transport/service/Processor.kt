package one.tesseract.transport.service

import java.util.concurrent.CompletionStage

class Processor(val native: Long) {
    external fun process(data: ByteArray): CompletionStage<ByteArray>

    protected external fun finalize()
}
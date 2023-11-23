package one.tesseract.client.java

import one.tesseract.client.transport.Status
import java.util.concurrent.CompletionStage

interface Delegate {
    fun selectTransport(transports: Map<String, Status>): CompletionStage<String?>
}

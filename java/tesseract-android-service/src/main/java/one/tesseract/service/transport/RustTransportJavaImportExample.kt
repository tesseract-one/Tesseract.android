package one.tesseract.service.transport

/*
This is an example, how to import rust implemented transports (i.e. WebSocket) into Java.
The most interesting part happens in the native implementation of createApplicator (check it out in Rust).

import one.tesseract.transport.service.RustTransport

class ExampleRustTransport(@Suppress("unused") val channel: String = "default"): RustTransport {
    external override fun createApplicator(): Long
}
 */
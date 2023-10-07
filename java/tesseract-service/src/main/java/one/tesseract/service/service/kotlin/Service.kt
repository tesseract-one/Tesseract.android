package one.tesseract.service.service.kotlin

import kotlinx.coroutines.CoroutineScope
import one.tesseract.service.service.java.Service as JavaService

interface Service {
    fun toJava(scope: CoroutineScope): JavaService
}
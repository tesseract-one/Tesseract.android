//===------------ RBiConsumer.kt --------------------------------------------===//
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

package one.tesseract.crabdroid

import java.util.function.BiConsumer

//double boxed closure
@Suppress("NewApi")
public class RBiConsumer<A, B>(val closure: Long): BiConsumer<A, B> {
    public external override fun accept(t: A, u: B)
    protected external fun finalize()
}
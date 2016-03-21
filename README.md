# Rumble
## Rust API for Mumble

This is a native rust library for the easy creation of bots or other software supporting mumble.

# Contributing

## Running the examples

You will need to generate a pair of certificate- and keyfile for the examples to work.
You may do so by running the supplied script: ```user@machine ~/rumble> ./generate_certificate```.

## Generating the protobuf bindings

In order to generate the protobuf bindings from a newer version of the mumble.proto file you will need to run:
```user@machine ~/rumble> protoc --rust_out src mumble.protoprotoc --rust_out src mumble.proto```.

Please note that the protobuf tools as well as [rust-protobuf](https://github.com/stepancheg/rust-protobuf) need to be installed on your system in order to do so.


# License

The MIT License (MIT)
Copyright (c) 2016 Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

# Contributors

 * alexd2580 (Alexander Dmitriev)
 * Prior99 (Frederick Gnodtke)

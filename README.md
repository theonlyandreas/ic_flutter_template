# Flutter template to call the Internet Computer

Flutter + Rust = Access to the Internet Computer = :heart:

This project is forked from [flutterust](https://github.com/shekohex/flutterust) and its method of executing Rust code from a Flutter application is applied to access the Internet Computer. 

For further documentation on how this template works with [Foreign Function Interfaces](https://en.wikipedia.org/wiki/Foreign_function_interface), see [Rust and Dart: the async story](https://dev.to/sunshine-chain/rust-and-dart-the-async-story-3adk).


## Project Structure

```
.
├── android
├── ios
├── lib                     <- The Flutter App Code
├── native                  <- Containes all the Rust Code
│   ├── ic
│   └── ic-ffi
├── packages                <- Containes all the Dart Packages that bind to the Rust Code
│   └── ic_ffi
├── target                  <- The compiled rust code for every arch
│   ├── aarch64-apple-ios
│   ├── aarch64-linux-android
│   ├── armv7-linux-androideabi
│   ├── debug
│   ├── i686-linux-android
│   ├── universal
│   ├── x86_64-apple-ios
│   └── x86_64-linux-android
└── test
```

## Setup and Tools

1. Install Rust and add build targets

#### For Android

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
```

#### For iOS

```sh
rustup target add aarch64-apple-ios x86_64-apple-ios
```

2. Cargo Plugins

```sh
cargo install --force cargo-make
```

we also use [`dart-bindgen`](https://github.com/sunshine-protocol/dart-bindgen) which requires LLVM/Clang. Install LLVM (10+) in the following way:

#### ubuntu/linux
1. Install libclangdev - `sudo apt-get install libclang-dev`.
2. Install openssl - `sudo apt-get install pkg-config libssl-dev`.

#### Windows
1. Install Visual Studio with C++ development support.
2. Install [LLVM](https://releases.llvm.org/download.html) or `winget install -e --id LLVM.LLVM`.

#### MacOS
1. Install Xcode.
2. Install LLVM - `brew install llvm`.


## How it works?

The simple idea here is that we build our rust code for all supported targets
then build a Flutter Package that uses these targets.

##### In iOS

we build our rust package using [`cargo-lipo`](https://github.com/TimNN/cargo-lipo) to build a universal iOS static lib from our rust code
after that, we symbol link the built library to our package ios directory, copy the generated `bindgen.h` file to the `ios/Classes`
the `Makefile.toml` do these steps for us.

Next we need to add these lines to our package podspec file:

```rb
  s.public_header_files = 'Classes**/*.h'
  s.static_framework = true
  s.vendored_libraries = "**/*.a"
```

but Xcode dose some tree shaking and we currently not using our static lib anywhere in the code, so we open our package's `ios/Classes/Swift{PACKAGE_NAME}Plugin.swift` and add a dummy method there:

```swift
 public static func dummyMethodToEnforceBundling() {
    // call some function from our static lib
    add(40, 2)
  }
```

##### In Android

In android it is a bit simpler than iOS, we just need to symbol link some libs in the right place and that is it.
our build script creates this folder structure for every package we have:

```
packages/{PACKAGE_NAME}/android/src/main
├── jniLibs
│   ├── arm64-v8a
│   ├── armeabi-v7a
│   └── x86
```

Make sure that the Android NDK is installed (From SDK Manager in Android Studio), also ensure that the env variable `$ANDROID_NDK_HOME` points to the NDK base folder
and after that, the build script build our rust crate for all of these targets using [`cargo-ndk`](https://github.com/bbqsrc/cargo-ndk)
and symbol link our rust lib to the right place, and it just works :)


## Build and Test

In the Root of the project simply run:

```sh
cargo make
```

Then run flutter app without null safety due to Dart's Isolate package

```
flutter run --no-sound-null-safety
```


## See also

- [Dart Meets Rust: a match made in heaven ✨](https://dev.to/sunshine-chain/dart-meets-rust-a-match-made-in-heaven-9f5)
- [Dart and Rust: the async story 🔃](https://dev.to/sunshine-chain/rust-and-dart-the-async-story-3adk)
- https://github.com/brickpop/flutter-rust-ffi
- https://dart.dev/guides/libraries/c-interop
- https://flutter.dev/docs/development/platform-integration/c-interop
- https://github.com/dart-lang/samples/blob/master/ffi/structs/structs.dart
- https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html
- https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html

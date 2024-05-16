# Android Rust interface example

This is an example library which demonstrates the interfacing between
Rust and an Android application. This library shows the perspective from
the Rust side. For information on the Android side and integration
hints see [this repository](https://github.com/santifa/android-rust).

## Rust perspective

For integrating a Rust library with Android certain steps are requried.

### Setup the toolchain

Setup the Rust development tools and toolchain:
``` shell
wget https://sh.rustup.rs -O rustup.sh
sh rustup.sh -y
# Add the Rust targets
rustup target add \
        aarch64-linux-android \
        armv7-linux-androideabi \
        i686-linux-android \
        x86_64-linux-android
rustup toolchain install nightly
rustup target add --toolchain nightly \
        aarch64-linux-android \
        armv7-linux-androideabi \
        i686-linux-android \
        x86_64-linux-android
```

Add the Rust specific target linkers to the `cargo` configuration file `~/.cargo/config.toml`:
``` toml
[target.aarch64-linux-android]
linker = "${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android30-clang"

[target.armv7-linux-androideabi]
linker = "${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi30-clang"

[target.i686-linux-android]
linker = "${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android30-clang"

[target.x86_64-linux-android]
linker = "${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android30-clang"
```

Maybe the `${NDK_HOME}` needs to be replaced with the full path.

The Rust library needs to be compiled as a shared object file. To do so, change the
compiler target within the project `Cargo.toml`:

``` toml
[lib]
crate-type = ["dylib"]
```

Adjust the `PATH` variable to allow cargo to find the relevant linker binaries:

``` shell
PATH=$PATH:${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin
```

No the relevant releases can be build with:

``` shell
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cargo build --target x86_64-linux-android --release
```

### Android integration

For the first part see the [Android project](https://github.com/santifa/android-rust).

The second part starts with adding the `jni` crate to the `Cargo.toml`:

``` toml
[target.'cfg(target_os="android")'.dependencies]
jni = { version = "0.20", default-features = false }
```

Within the code an extra module can be used to distinguish the Android part:

``` rust
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
 // imports
 
 #[no_mangle]
    pub unsafe extern "system" fn Java_com_santifa_android_1rust_NativeLibrary_helloWorld<
        'local,
    >(
        env: JNIEnv<'local>,
        _: JClass<'local>,
    ) -> jstring {
        let output = env
            .new_string(print_hello())
            .expect("Couldn't create java string!");
        output.into_raw()
    }
}
```

The `#[no_mangle]` declarative tells Rust to keep the function names as is and doesn't
change them to make them individual within the library.

### Function names

The function names are somewhat special with the schemativ
`Java_<package>_<class>_<function>`. To get them right the Java compiler can be used:
`javac -h . java/com/example/android_rust/NativeLibrary.java`. This generates a C header
file with the correct namings.

Listing symbols in a shared library: `nm -gDC yourLib.so`

## Demonstration Cases

The example implements some basic demonstration cases.

1. The first case is a simple call to the Rust native library and returns a JVM string with the architecture
2. The second use case is a simple addition with left and right hand side.
3. The third use case is a more complex operation printing the first perfect numbers up-to the input number. 
See [this](https://rosettacode.org/wiki/Perfect_numbers) one for more informations

## References

The following references were used:
* [Rust Library Android](https://gendignoux.com/blog/2022/10/24/rust-library-android.html)
* [Rust JNI](https://docs.rs/jni/latest/jni/)
* [Rust on Android](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)
* [Building Rust Modules](https://source.android.com/docs/setup/build/rust/building-rust-modules/overview?hl=de)

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    use crate::native::add;
    use crate::native::print_hello;
    use jni::objects::JClass;
    use jni::sys::{jint, jstring};
    use jni::JNIEnv;

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

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_santifa_android_1rust_NativeLibrary_add<'local>(
        _env: JNIEnv<'local>,
        _: JClass<'local>,
        left: jint,
        right: jint,
    ) -> jint {
        let result = add(left as i32, right as i32);
        result as jint
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_santifa_android_1rust_NativeLibrary_perfectNumbers<
        'local,
    >(
        env: JNIEnv<'local>,
        _: JClass<'local>,
        input: jint,
    ) -> jstring {
        let result = String::from("Nothin implemented");
        let output = env
            .new_string(result)
            .expect("Couldn't create java string!");
        output.into_raw()
    }
}

pub mod native {
    pub fn add(left: i32, right: i32) -> i32 {
        left + right
    }

    pub fn print_hello() -> String {
        format!(
            "Hello world from rust! You're running a {}",
            get_arch_name()
        )
    }

    // Determine the architecture
    fn get_arch_name() -> &'static str {
        #[cfg(target_arch = "x86")]
        return "x86";

        #[cfg(target_arch = "x86_64")]
        return "x86_64";

        #[cfg(target_arch = "arm")]
        return "arm";

        #[cfg(target_arch = "aarch64")]
        return "aarch64";

        #[cfg(not(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "arm",
            target_arch = "aarch64",
        )))]
        return "unknown";
    }
}

#[cfg(test)]
mod tests {
    use crate::native::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

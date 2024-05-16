#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    use crate::native::{add, perfect_numbers, print_hello};
    use jni::objects::{JClass, JObject, JString, JValue};
    use jni::sys::{jint, jintArray, jstring};
    use jni::JNIEnv;

    struct AndroidLogger<'local> {
        /// Reference to the android.util.Log class.
        log_class: JClass<'local>,
        /// Tag for log messages.
        tag: JString<'local>,
    }

    impl<'local> AndroidLogger<'local> {
        pub fn new(env: &mut JNIEnv<'local>, tag: &str) -> Result<Self, jni::errors::Error> {
            Ok(Self {
                log_class: env.find_class("android/util/Log")?,
                tag: env.new_string(tag)?,
            })
        }

        /// Prints a message at the debug level.
        pub fn d(
            &self,
            env: &mut JNIEnv<'local>,
            message: impl AsRef<str>,
        ) -> Result<(), jni::errors::Error> {
            env.call_static_method(
                &self.log_class,
                "d",
                "(Ljava/lang/String;Ljava/lang/String;)I",
                &[
                    JValue::Object(&self.tag),
                    JValue::Object(&JObject::from(env.new_string(message)?)),
                ],
            )?;
            Ok(())
        }
    }

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
        mut env: JNIEnv<'local>,
        _: JClass<'local>,
        input: jint,
    ) -> jintArray {
        let logger =
            AndroidLogger::new(&mut env, "android_rust_lib").expect("Could not build logger");
        let _ = logger.d(&mut env, "Called rust lib");
        let numbers = perfect_numbers(input);
        let _ = logger.d(&mut env, "Calculated perfect numbers");
        let int_array = env
            .new_int_array(numbers.len() as i32)
            .expect("Failed to generate array");
        let _ = logger.d(&mut env, "Converted numbers to Java array");
        let _ = env.set_int_array_region(&int_array, 0, &numbers);
        int_array.into_raw()
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

    pub fn perfect_numbers(input: i32) -> Vec<i32> {
        (2..input + 1)
            .filter(|x| x % 2 != 1)
            .filter(is_perfect)
            .collect()
    }

    fn is_perfect(n: &i32) -> bool {
        let mut tot = 1;
        let mut q: i32;

        for i in 2..(*n as f32).sqrt() as i32 + 1 {
            if n % i == 0 {
                tot += i;
                q = n / i;
                if q > i {
                    tot += q
                }
            }
        }
        tot == *n
    }
}

#[cfg(test)]
mod tests {
    use crate::native::*;

    #[test]
    fn test_perfect_numbers() {
        let numbers = perfect_numbers(100);
        assert_eq!(numbers, vec![6, 28])
    }

    #[test]
    fn test_perfect_numbers_huge() {
        let numbers = perfect_numbers(33550336);
        assert_eq!(numbers, vec![6, 28, 496, 8128, 33550336])
    }
}

#![allow(non_snake_case)]
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

// This function name strictly follows the JNI naming convention:
// Java_<PackageName>_<ClassName>_<MethodName>
#[no_mangle]
pub extern "system" fn Java_com_remotelinkdesk_app_MainActivity_getCoreVersion<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    // We will return a string to prove the Rust engine is properly loaded
    let output = env.new_string("RemoteLinkDesk Rust Core v0.1.0 (Loaded via JNI!)")
        .expect("Couldn't create java string!");
    
    output.into_raw()
}

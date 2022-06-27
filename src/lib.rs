use casbin::prelude::*;
use futures::executor::block_on;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// async fn enforce() -> Result<bool> {
//     let mut e = Enforcer::new(
//         "examples/rbac_with_domains_model.conf",
//         "examples/rbac_with_domains_policy.csv",
//     )
//     .await?;
//     e.enable_log(true);
//
//     return e.enforce(("alice", "domain1", "data1", "read"));
// }

#[no_mangle]
pub extern "C" fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Ok(string) => string,
        Err(_) => "Error",
    };

    return CString::new("RAWR @ ".to_owned() + recipient)
        .unwrap()
        .into_raw();
}

async fn enforce(model: &str, lines: Vec<String>) -> String {
    // load the model
    let m = DefaultModel::from_str(model).await.unwrap();

    let a = MemoryAdapter::default();

    let mut e = Enforcer::new(m, a).await.unwrap();

    let things = lines
        .iter()
        .map(move |line| -> Vec<String> {
            line.split(&[',', ' ']).map(|c| c.to_string()).collect()
        })
        .collect();

    let _ = e.add_policies(things).await.unwrap();

    match e.enforce(("a", "b", "c")) {
        Ok(true) => "YAY".to_string(),
        Ok(false) => "BOO".to_string(),
        Err(e) => format!("BAD: {e:?}"),
    }
}

// #[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jobjectArray, jstring};
    use self::jni::JNIEnv;
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_sevenshifts_libsevenclient_AccessControlList_enforce(
        env: JNIEnv,
        _: JClass,
        jmodel_text: JString,
        jpolicy_lines: jobjectArray,
        _jrequest: jobjectArray,
    ) -> jstring {
        let model: String = env
            .get_string(jmodel_text)
            .expect("couldn't get model string")
            .into();

        let mut lines = Vec::new();

        let count = env.get_array_length(jpolicy_lines).unwrap();
        for n in 0..count {
            let line_obj = env.get_object_array_element(jpolicy_lines, n).unwrap();
            let line_obj2 = JString::from(line_obj);
            let line: String = env.get_string(line_obj2).unwrap().into();

            lines.push(line);
        }

        let result = block_on(enforce(model.as_str(), lines));

        let result_ptr = CString::new(result).unwrap();
        // let result_ptr = CString::from_raw(result);
        let jresult = env
            .new_string(result_ptr.to_str().unwrap())
            .expect("couldn't create java script");
        return jresult.into_inner();
    }
}

use casbin::prelude::*;
use futures::executor::block_on;
use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

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

struct NamedPolicy {
    ptype: String,
    elements: Vec<String>,
}

async fn enforce(
    model: String,
    policy: String,
    subject_json: String,
    organizational_unit: String,
    resource: String,
    action: String,
    attrs: BTreeMap<String, String>,
) -> String {
    // load the model
    let m = DefaultModel::from_str(model.as_str()).await.unwrap();
    let a = MemoryAdapter::default();

    let mut e = Enforcer::new(m, a).await.unwrap();

    let named_policies: Vec<NamedPolicy> = policy
        .lines()
        .map(move |line| {
            let tokens: Vec<String> = line.split(&[',']).map(|c| c.trim().to_string()).collect();

            let (ptype, elements) = tokens.split_first().unwrap();

            return NamedPolicy {
                ptype: ptype.to_string(),
                elements: elements.iter().map(|p| p.to_string()).collect(),
            };
        })
        .collect();

    for named_policy in named_policies {
        match named_policy.ptype.as_str() {
            "p" => {
                let _ = e
                    .add_named_policy("p", named_policy.elements)
                    .await
                    .unwrap();
            }

            g => {
                let _ = e
                    .add_named_grouping_policy(g, named_policy.elements)
                    .await
                    .unwrap();
            }
        }
    }

    let subject: BTreeMap<String, String> = serde_json::from_str(&subject_json).expect("derp");

    let request = (subject, organizational_unit, resource, action, attrs);

    return match e.enforce(request) {
        Ok(true) => "YAY".to_string(),
        Ok(false) => "BOO".to_string(),
        Err(e) => format!("BAD: {e:?}"),
    };
}

// #[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_sevenshifts_libsevenclient_AccessControlList_enforce(
        env: JNIEnv,
        _: JClass,
        jmodel: JString,
        jpolicy: JString,
        jsubject_json: JString,
        jorganizational_unit: JString,
        jresource: JString,
        jaction: JString,
    ) -> jstring {
        let model: String = env
            .get_string(jmodel)
            .expect("couldn't get model string")
            .into();

        let policy: String = env
            .get_string(jpolicy)
            .expect("couldn't get policy string")
            .into();

        let subject_json = env
            .get_string(jsubject_json)
            .expect("couldn't get subject JSON string")
            .into();

        let ou: String = env
            .get_string(jorganizational_unit)
            .expect("couldn't get organizational unit string")
            .into();

        let resource: String = env
            .get_string(jresource)
            .expect("couldn't get resource string")
            .into();

        let action: String = env
            .get_string(jaction)
            .expect("couldn't get action string")
            .into();

        let attrs: BTreeMap<String, String> = BTreeMap::new();

        let result = block_on(enforce(
            model,
            policy,
            subject_json,
            ou,
            resource,
            action,
            attrs,
        ));

        let result_ptr = CString::new(result).unwrap();
        let jresult = env
            .new_string(result_ptr.to_str().unwrap())
            .expect("couldn't create java script");
        return jresult.into_inner();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seven_shifts_model() {
        let model = r#"
            [request_definition]
            r = sub, ou, resource, act, attrs

            [policy_definition]
            p = sub, ou, resource, act, org, sub_rule

            [role_definition]
            g  = _, _, _
            g2 = _, _, _

            [policy_effect]
            e = some(where (p.eft == allow))

            [matchers]
            m = g(r.sub.id, p.sub, r.sub.org) && g2(r.ou, p.ou, r.sub.org) && p.org == r.sub.org && keyMatch(r.resource, p.resource) && keyMatch(r.act, p.act) && eval(p.sub_rule)
            "#.to_string();

        let policy = r#"
            p, company:11_user_write, company:11, user, write, org:11, 1 < 2
            p, department:33_user_write, department:33, user, write, org:11, 1 < 2
            p, location:22_user_write, location:22, user, write, org:11, 1 < 2
            p, company:11_user_read, company:11, user, read, org:11, 1 < 2
            g, 55, company:11_user_read, org:11
            g, 55, location:22_user_write, org:11
            g2, location:22, company:11, org:11
            g2, location:23, company:11, org:11
            g2, department:33, location:22, org:11
        "#
        .to_string();

        let subject_json = r#"{"id": "55", "org": "org:11"}"#.to_string();

        let attrs = BTreeMap::<String, String>::new();

        let organizational_unit = "company:11".to_string();
        let resource = "user".to_string();
        let action = "read".to_string();

        let result = block_on(enforce(
            model,
            policy,
            subject_json,
            organizational_unit,
            resource,
            action,
            attrs,
        ));
        assert_eq!(result, "YAY");
    }
}

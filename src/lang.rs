use hbb_common::regex::Regex;
use std::ops::Deref;

mod ptbr;

pub const LANGS: &[(&str, &str)] = &[
    ("pt", "Português"),
];

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn translate(name: String) -> String {
    let locale = sys_locale::get_locale().unwrap_or_default();
    translate_locale(name, &locale)
}

pub fn translate_locale(name: String, locale: &str) -> String {
    let locale = locale.to_lowercase();
    let mut lang = hbb_common::config::LocalConfig::get_option("lang").to_lowercase();
    if lang.is_empty() {
        lang = locale
            .split('-')
            .next()
            .map(|x| x.split('_').next().unwrap_or_default())
            .unwrap_or_default()
            .to_owned();
    }

    let lang = lang.to_lowercase();
    let m = match lang.as_str() {
        "pt" | "br" => ptbr::T.deref(),
        _ => ptbr::T.deref(), // padrão: português
    };

    let (name, placeholder_value) = extract_placeholder(&name);
    let replace = |s: &&str| {
        let mut s = s.to_string();
        if let Some(value) = placeholder_value.as_ref() {
            s = s.replace("{}", &value);
        }
        if !crate::is_rustdesk() {
            if s.contains("RustDesk")
                && !name.starts_with("upgrade_rustdesk_server_pro")
                && name != "powered_by_me"
            {
                s = s.replace("RustDesk", &crate::get_app_name());
            }
        }
        s
    };

    if let Some(v) = m.get(&name as &str) {
        if !v.is_empty() {
            return replace(v);
        }
    }

    replace(&name.as_str())
}

fn extract_placeholder(input: &str) -> (String, Option<String>) {
    if let Ok(re) = Regex::new(r#"\{(.*?)\}"#) {
        if let Some(captures) = re.captures(input) {
            if let Some(inner_match) = captures.get(1) {
                let name = re.replace(input, "{}").to_string();
                let value = inner_match.as_str().to_string();
                return (name, Some(value));
            }
        }
    }
    (input.to_string(), None)
}

mod test {
    #[test]
    fn test_extract_placeholders() {
        use super::extract_placeholder as f;

        assert_eq!(f(""), ("".to_string(), None));
        assert_eq!(
            f("{3} sessions"),
            ("{} sessions".to_string(), Some("3".to_string()))
        );
        assert_eq!(f(" } { "), (" } { ".to_string(), None));
        assert_eq!(
            f("{} sessions"),
            ("{} sessions".to_string(), Some("".to_string()))
        );
        assert_eq!(
            f("{2} times {4} makes {8}"),
            ("{} times {4} makes {8}".to_string(), Some("2".to_string()))
        );
    }
}

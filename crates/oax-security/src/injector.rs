use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CredentialInjector {
    secrets: HashMap<String, String>,
}

impl CredentialInjector {
    pub fn new(secrets: HashMap<String, String>) -> Self {
        Self { secrets }
    }

    pub fn inject(&self, input: &str) -> Result<String, Vec<String>> {
        let mut out = String::new();
        let mut missing = Vec::new();
        let mut remaining = input;

        while let Some(start) = remaining.find("${") {
            out.push_str(&remaining[..start]);
            let after = &remaining[start + 2..];
            let Some(end) = after.find('}') else {
                out.push_str(&remaining[start..]);
                break;
            };

            let key = &after[..end];
            if let Some(value) = self.secrets.get(key) {
                out.push_str(value);
            } else if !missing.iter().any(|m| m == key) {
                missing.push(key.to_string());
            }

            remaining = &after[end + 1..];
        }

        out.push_str(remaining);

        if missing.is_empty() {
            Ok(out)
        } else {
            Err(missing)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CredentialInjector;
    use std::collections::HashMap;

    #[test]
    fn injects_secret_placeholders() {
        let mut secrets = HashMap::new();
        secrets.insert("GITHUB_TOKEN".to_string(), "ghp_secret".to_string());
        let injector = CredentialInjector::new(secrets);

        let output = injector
            .inject("Authorization: Bearer ${GITHUB_TOKEN}")
            .expect("injection should succeed");

        assert_eq!(output, "Authorization: Bearer ghp_secret");
    }

    #[test]
    fn reports_missing_secret_names() {
        let injector = CredentialInjector::new(HashMap::new());
        let err = injector
            .inject("Authorization: Bearer ${MISSING_TOKEN}")
            .expect_err("missing secret should error");

        assert_eq!(err, vec!["MISSING_TOKEN".to_string()]);
    }
}

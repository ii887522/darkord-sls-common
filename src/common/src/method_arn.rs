#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MethodArn {
    pub api_arn: String,
    pub stage: String,
    pub method: String,
    pub path: String,
}

impl From<&str> for MethodArn {
    fn from(method_arn: &str) -> Self {
        let mut method_arn = method_arn.split('/');

        Self {
            api_arn: method_arn.next().unwrap().to_string(),
            stage: method_arn.next().unwrap().to_string(),
            method: method_arn.next().unwrap().to_string(),
            path: method_arn.collect::<Vec<_>>().join("/"),
        }
    }
}

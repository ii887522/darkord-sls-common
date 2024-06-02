use serde::Deserialize;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    Post,
    Put,
    Get,
    Delete,
}

use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use uuid::Uuid;

#[derive(
    Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    opg::OpgModel,
    sqlx::Type,
)]
#[opg("Service UUID (v4)")]
#[sqlx(transparent)]
pub struct ServiceId(pub Uuid);

impl ServiceId {
    pub fn new(id: Uuid) -> Self {
        ServiceId(id)
    }

    pub fn inner(&self) -> &Uuid {
        &self.0
    }

    #[allow(dead_code)]
    pub fn generate() -> Self {
        ServiceId(Uuid::new_v4())
    }
}

impl FromStr for ServiceId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Uuid::parse_str(s)?;
        Ok(ServiceId::new(id))
    }
}

impl Display for ServiceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{}", self.0))
    }
}

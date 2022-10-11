use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::AsRefStr;
use strum_macros::Display;
use strum_macros::IntoStaticStr;

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ToggleProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "result")]
    RESULT,
}

impl ToggleProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ToggleProperties::TRIGGER => json!(false),
            ToggleProperties::RESULT => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(ToggleProperties::TRIGGER), NamedProperty::from(ToggleProperties::RESULT)]
    }
}

impl From<ToggleProperties> for NamedProperty {
    fn from(p: ToggleProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<ToggleProperties> for String {
    fn from(p: ToggleProperties) -> Self {
        p.to_string()
    }
}

use serde::ser::SerializeMap;
use serde::{de::MapAccess, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug)]
pub struct Project {
    pub path: String,
    pub title: String,
}

impl Project {
    pub fn new(path: String, title: String) -> Project {
        Project { path, title }
    }
}

impl Serialize for Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_map(Some(2))?;
        seq.serialize_entry("title", &self.title)?;
        seq.serialize_entry("path", &self.path)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Project {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        //deserializer.deserialize_any(CustomVisitor)
        deserializer.deserialize_map(CustomVisitor)
    }
}

struct CustomVisitor;

impl<'de> Visitor<'de> for CustomVisitor {
    type Value = Project;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut title = None;
        let mut path = None;

        while let Some(k) = map.next_key::<&str>()? {
            if k == "title" {
                title = Some(map.next_value()?);
            } else if k == "path" {
                path = Some(map.next_value()?);
            } else {
                return Err(serde::de::Error::custom(&format!("Invalid key: {}", k)));
            }
        }

        if title.is_none() || path.is_none() {
            return Err(serde::de::Error::custom("Missing first or second"));
        };
        return Ok(Project::new(title.unwrap(), path.unwrap()));
    }
}

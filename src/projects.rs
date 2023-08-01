use serde::ser::SerializeMap;
use serde::{de::MapAccess, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::{env, fs};

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
            return Err(serde::de::Error::custom("Missing title or path"));
        };
        return Ok(Project::new(path.unwrap(), title.unwrap()));
    }
}

pub fn get_project() -> Project {
    let args: Vec<String> = env::args().collect();
    // require at least 2 args
    if args.len() < 2 {
        panic!("not enough arguments");
    };
    let mut path = args[1].clone();
    if path == "." {
        path = env::current_dir().unwrap().to_str().unwrap().to_string();
    };
    let title = args[2].clone();
    Project::new(path, title)
}

pub fn mark() {
    let project = get_project();
    write_json(project);
}

pub fn check_file() -> bool {
    // check if project.json exists
    let file = String::from("projects.json");
    match fs::metadata(file) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn write_json(project: Project) {
    if !check_file() {
        fs::write("projects.json", "").expect("unable to write file");
    }
    let contents =
        fs::read_to_string("projects.json").expect("Should have been able to read the file");
    // create new array if content is empty
    if contents == "" {
        let projects: Vec<Project> = vec![project];
        let json = serde_json::to_string(&projects).unwrap();
        fs::write("projects.json", json).expect("Unable to write file");
        return;
    }

    let mut projects: Vec<Project> = serde_json::from_str(&contents).unwrap();

    // check if project exist in projects
    if projects.iter().any(|p| p.path == project.path) {
        println!("Project with path - {} already exist", project.path);
        return;
    }
    projects.push(project);
    let json = serde_json::to_string(&projects).unwrap();
    fs::write("projects.json", json).expect("Unable to write file");
}

pub fn read_projects() -> Vec<Project> {
    if !check_file() {
        return vec![];
    };
    let contents =
        fs::read_to_string("projects.json").expect("Should have been able to read the file");
    let projects: Vec<Project> = serde_json::from_str(&contents).unwrap();
    projects
}

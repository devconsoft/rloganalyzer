
use definition::rule::Rule;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq)]
pub struct Definition {
    pub title: String,
    pub id: String,
    pub desc: String,
    #[serde(default)]
    pub markers: Vec<Rule>,
    #[serde(default)]
    pub invalidators: Vec<Rule>,
    #[serde(default)]
    pub examples: Vec<String>,
}

impl Definition {
    pub fn new(title: &String, id: &String, desc: &String) -> Definition {
        Definition {
            title: title.clone(),
            id: id.clone(),
            desc: desc.clone(),
            markers: Vec::new(),
            invalidators: Vec::new(),
            examples: Vec::new(),
        }
    }

    pub fn add_marker(&mut self, pattern: &String) {
        self.markers.push(Rule::new(pattern));
    }

    pub fn add_invalidator(&mut self, pattern: &String) {
        self.invalidators.push(Rule::new(pattern));
    }

    pub fn add_example(&mut self, example: String) {
        self.examples.push(example);
    }

}

#[cfg(test)]
mod tests {

    use definition::definition::Definition;

    #[test]
    fn definition_new() {
        let def = Definition::new(
            &"title".to_string(),
            &"id".to_string(),
            &"desc".to_string(),
        );
        assert_eq!(def.title, "title");
        assert_eq!(def.id, "id");
        assert_eq!(def.desc, "desc");
    }
}

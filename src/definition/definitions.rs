
use definition::definition::Definition;
use serde_yaml;
use std::fs::File;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq)]
pub struct Definitions {
    definitions : Vec<Definition>,
}

impl Definitions {

    pub fn new() -> Definitions {
        Definitions {
            definitions: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    pub fn add(&mut self, elem: Definition) {
        self.definitions.push(elem);
    }

    pub fn get(&self, index: usize) -> &Definition {
        &self.definitions[index]
    }

    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    pub fn from_yaml(filename: &str) -> Definitions {
        let f = File::open(filename).expect("file not found");
        serde_yaml::from_reader(f).unwrap()
    }
}

impl IntoIterator for Definitions {
    type Item = Definition;
    type IntoIter = ::std::vec::IntoIter<Definition>;

    fn into_iter(self) -> Self::IntoIter {
        self.definitions.into_iter()
    }
}

#[cfg(test)]
mod tests {

    use definition::definitions::Definitions;
    use definition::definition::Definition;
    use unindent::unindent;

    fn get_definitions() -> Definitions {
        let mut defs = Definitions::new();
        defs.add(
            Definition::new(
                &"title".to_string(),
                &"id".to_string(),
                &"desc".to_string(),
            )
        );
        return defs
    }

    #[test]
    fn new_definitions() {
        let defs = Definitions::new();
        assert_eq!(defs.len(), 0);
    }

    #[test]
    fn add_to_definitions() {
        let defs = get_definitions();
        assert_eq!(defs.len(), 1);
    }

    #[test]
    fn get_from_definitions() {
        let defs = get_definitions();
        assert_eq!(defs.get(0), defs.get(0));
    }

    #[test]
    fn iter_definitions() {
        let defs = get_definitions();
        let mut counter = 0;
        for _d in defs.into_iter() {
            counter += 1;
        }
        assert_eq!(counter, 1);
    }

    #[test]
    fn definitions_to_yaml() {
        let mut defs = Definitions::new();
        let mut d1 = Definition::new(
            &"title1".to_string(),
            &"id1".to_string(),
            &"desc1".to_string(),
        );
        d1.add_marker(&"marker.*".to_string());
        d1.add_invalidator(&"invalidator.*".to_string());
        d1.add_example("example".to_string());

        defs.add(d1);
        defs.add(
            Definition::new(
                &"title2".to_string(),
                &"id2".to_string(),
                &"desc2".to_string(),
            )
        );
        let yaml = defs.to_yaml();
        let expected = unindent(r#"
        ---
        definitions:
          - title: title1
            id: id1
            desc: desc1
            markers:
              - "marker.*"
            invalidators:
              - "invalidator.*"
            examples:
              - example
          - title: title2
            id: id2
            desc: desc2
            markers: []
            invalidators: []
            examples: []
            "#);
        assert_eq!(expected.trim(), yaml.trim());
    }

}

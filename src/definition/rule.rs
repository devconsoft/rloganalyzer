use regex::Regex;
use std::fmt;
use serde::de::{Visitor, Error};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub struct Rule {
    pub pattern: Regex,
}

impl Rule {
    pub fn new(pattern: &str) -> Rule {
        Rule {
            pattern: Regex::new(pattern).unwrap(),
        }
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Rule) -> bool {
        self.pattern.as_str() == other.pattern.as_str()
    }
}
impl Eq for Rule {}

impl Serialize for Rule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.pattern.as_str())
    }
}

struct RuleVisitor;

impl<'a> Visitor<'a> for RuleVisitor {
    type Value = Rule;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("valid regular expression")
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: Error
    {
        Ok(Rule::new(value))
    }
}

impl<'de> Deserialize<'de> for Rule {
    fn deserialize<D>(deserializer: D) -> Result<Rule, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(RuleVisitor)
    }
}


#[cfg(test)]
mod tests {

    use definition::rule::Rule;

    #[test]
    fn rule_new() {
        let _r = Rule::new(&"w+");
    }

    #[test]
    fn rule_eq() {
        let r1 = Rule::new(&"w+");
        let r2 = Rule::new(&"w+");
        assert_eq!(r1,r2);
    }

}

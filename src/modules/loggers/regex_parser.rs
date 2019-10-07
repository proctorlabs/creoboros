use super::*;

use regex::Regex;

#[derive(Debug)]
pub struct RegexParser {
    name: String,
    regex: Regex,
    forward_to: String,
}

impl RegexParser {
    pub fn new(name: String, pattern: String, forward_to: String) -> Result<Self> {
        Ok(RegexParser {
            name,
            regex: Regex::new(&pattern)?,
            forward_to,
        })
    }
}

impl ModuleExt for RegexParser {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle(&self, mut m: Message) -> Result<()> {
        if let Message::Log { ref mut log } = m {
            let raw_log: String = log["log"].to_string();
            if self.regex.is_match(&raw_log) {
                let caps = self.regex.captures(&raw_log).unwrap();
                for g in self.regex.capture_names().filter_map(|e| e) {
                    if let Some(val) = caps.name(g) {
                        log[g] = val.as_str().into();
                    }
                }
            }
        }
        CERBERUS.send(&self.forward_to, m);
        Ok(())
    }
}

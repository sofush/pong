use std::collections::HashSet;

#[derive(Default, Clone, Debug)]
pub struct Keyboard {
    key_states: HashSet<String>,
}

impl Keyboard {
    pub fn press(&mut self, key: impl Into<String>) {
        let key: String = key.into();
        log::debug!("User pressed a key: {:?}", key);
        self.key_states.insert(key);
    }

    pub fn release(&mut self, key: impl Into<String>) {
        let key: String = key.into();
        log::debug!("User released a key: {:?}", key);
        self.key_states.remove(&key);
    }

    pub fn is_any_pressed(&self, keys: &[&'static str]) -> bool {
        for key in keys.iter() {
            let s: String = key.to_string().to_ascii_lowercase();

            if self.key_states.contains(&s) {
                return true;
            }
        }

        false
    }
}

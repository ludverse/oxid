use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Memory<T> {
    scopes: Vec<HashMap<String, T>>
}

impl<T> Memory<T> {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()]
        }
    }

    pub fn insert(&mut self, key: String, value: T) -> Option<T> {
        self.scopes.last_mut().unwrap().insert(key, value)
    }

    pub fn assign(&mut self, key: String, value: T) -> Option<T> {
        for scope in self.scopes.iter_mut() {
            if scope.contains_key(&key) {
                return scope.insert(key, value);
            }
        }

        None
    }

    pub fn get(&self, key: &String) -> Option<&T> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(key))
    }

    pub fn has(&self, key: &String) -> bool {
        self.get(key).is_some()
    }

    pub fn scoped<F>(&mut self, f: F)
    where
        F: FnOnce()
    {
        self.push_scope();

        f();

        self.pop_scope();
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop().unwrap();
    }
}

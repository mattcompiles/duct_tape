use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};

use crate::js_module::JsModule;

pub struct ModuleGraph {
    modules: HashMap<String, JsModule>,
}

impl ModuleGraph {
    pub fn new() -> ModuleGraph {
        ModuleGraph {
            modules: HashMap::default(),
        }
    }

    pub fn load_module(&mut self, module: JsModule) {
        for dep in &module.dependencies {
            let dep_module = JsModule::new(dep).unwrap();

            self.load_module(dep_module);
        }

        self.modules.insert(module.filename.clone(), module);
    }
}

impl Debug for ModuleGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut modules: Vec<String> = vec![];
        self.modules.iter().for_each(|(module_filename, _module)| {
            modules.push(String::from(module_filename));
        });
        write!(f, "ModuleGraph ( modules: {} )", modules.join(", "))
    }
}
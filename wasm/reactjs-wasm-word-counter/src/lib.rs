mod utils;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Count {
    values: HashMap<String, i32>,
    no_of_values: i32
}

impl Count {
    pub fn new(values: HashMap<String, i32>, no_of_values: i32) -> Count {
        Count {
            values: values,
            no_of_values: no_of_values
        }
    }

    pub fn get_values(&self) -> HashMap<String, i32> {
        self.values.clone()
    }

    pub fn set_values(&mut self, values: HashMap<String, i32>) {
        self.values = values;
    }

    pub fn get_no_values(&self) -> i32 {
        self.no_of_values
    }

    pub fn set_no_values(&mut self, values: i32) {
        self.no_of_values = values;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let greet_string = format!("Hello {:?}!", name);
    alert(&greet_string);
}

#[wasm_bindgen]
pub fn count_words(i: &str) -> JsValue {
    let mut values = HashMap::new();
    let mut no_of_values = 0;

    let input = i.to_string().replace("\"", "")
        .replace(".", "")
        .replace("!", "")
        .replace("'", "")
        .replace(",", "");

    for word in input.split(' ') {
        no_of_values += 1;
        if word.len() > 0 {
            if values.contains_key(word) {
                if let Some(word) = values.get_mut(word) {
                    *word += 1i32;
                }
            } else {
                values.insert(word.to_string(), 1i32);
            }
        }
    }

    let ret = Count {
       values: values.clone(),
       no_of_values: no_of_values
   };

   JsValue::from_serde(&ret).unwrap()
}

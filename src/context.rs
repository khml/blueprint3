use std::collections::HashMap;
use std::cell::Cell;

pub struct Context {
    pub var_tbl: Cell<HashMap<String, f64>>
}

impl Context {
    pub fn new() -> Context {
        let tbl: HashMap<String, f64> = HashMap::new();
        Context { var_tbl: Cell::new(tbl) }
    }
}
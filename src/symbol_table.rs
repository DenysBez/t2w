use std::collections::HashSet;

#[derive(Debug)]
pub struct SymbolTable {
    pub symbol_table: HashSet<String>,
    pub labels_declared: HashSet<String>,
    pub labels_goto_ed: HashSet<String>
}

impl Default for SymbolTable {
    fn default() -> Self {
        SymbolTable {
            symbol_table: HashSet::new(),
            labels_declared: HashSet::new(),
            labels_goto_ed: HashSet::new()
        }
    }
}
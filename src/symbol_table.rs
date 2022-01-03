pub mod symbol_table {
    use std::collections::HashSet;

    #[derive(Debug)]
    pub struct Symbol_Table {
        pub symbol_table: HashSet<String>,
        pub labels_declared: HashSet<String>,
        pub labels_goto_ed: HashSet<String>
    }

    impl Default for Symbol_Table {
        fn default() -> Self {
            Symbol_Table {
                symbol_table: HashSet::new(),
                labels_declared: HashSet::new(),
                labels_goto_ed: HashSet::new()
            }
        }
    }

    impl Symbol_Table {
        fn add_labels_declared(&mut self, val: String) {
            self.labels_declared.insert(val);
        }

        fn add_labels_goto_ed(&mut self, val: String) {
            self.labels_goto_ed.insert(val);
        }

        fn add_symbol_table(&mut self, val: String) {
            self.symbol_table.insert(val);
        }
    }
}
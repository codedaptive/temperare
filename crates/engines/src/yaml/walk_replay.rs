//! `Walk::replay`: an alias given as the node it names. Tested through yaml_entries (yaml_entries_tests.rs), so no test
//! file of its own.

use super::walk::Walk;
use super::YamlEntry;

impl Walk<'_> {
    /// Gives the anchored node `id` again at `path`, an item or not, every entry at `line`, the alias's. The parser
    /// rejects an alias to an anchor it has not seen, and every anchored node and key is kept once read, so the one
    /// anchor not kept is a node still being read: an alias inside the node its anchor names.
    pub(super) fn replay(&mut self, id: usize, path: &[String], item: bool, line: usize) -> Result<(), String> {
        let inside = || format!("line {line}: an alias inside the node its anchor names");
        let (value, under) = self.anchors.get(&id).ok_or_else(inside)?;
        self.entries.push(YamlEntry { path: path.to_vec(), item, line, value: value.clone() });
        for (below, item, value) in under {
            self.entries.push(YamlEntry { path: [path, below].concat(), item: *item, line, value: value.clone() });
        }
        Ok(())
    }
}

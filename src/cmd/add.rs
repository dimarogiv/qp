
use anyhow::Result;

use crate::cmd::{Add, Run};
use crate::db::Database;
use crate::{config, util};

impl Run for Add {
    fn run(&self) -> Result<()> {
        // These characters can't be printed cleanly to a single line, so they can cause
        // confusion when writing to stdout.
        const EXCLUDE_CHARS: &[char] = &['\n', '\r'];

        let exclude_dirs = config::exclude_dirs()?;
        let max_age = config::maxage()?;
        let now = util::current_time()?;

        let mut db = Database::open()?;

        for path in &self.paths {
            let path =
                if config::resolve_symlinks() { util::canonicalize } else { util::resolve_path }(
                    path,
                )?;
            let path = util::path_to_str(&path)?;

            // Ignore path if it contains unsupported characters, or if it's in the exclude
            // list.
            if path.contains(EXCLUDE_CHARS) || exclude_dirs.iter().any(|glob| glob.matches(path)) {
                continue;
            }

            let by = self.score.unwrap_or(1.0);
            db.add_update(path, by, now);
        }

        if db.dirty() {
            db.age(max_age);
        }
        db.save()
    }
}

use crate::utils::find_zk_root;
use std::path::PathBuf;
use mdbook::MDBook;
use mdbook_katex::KatexProcessor;
use mdbook_backlinks::Backlinks;
use wikilink::AutoTitle;
use failure::{Error, err_msg};

pub fn build(dir: Option<PathBuf>) -> Result<(), Error> {
    let path = match dir {
        Some(path) => path,
        None => find_zk_root().ok_or(err_msg("Could not find the root of your Zettelkasten"))?,
    };

    let mut md = match MDBook::load(path) {
        Ok(val) => val,
        Err(e) => return Err(err_msg(e.to_string())),
    };

    md.with_preprocessor(KatexProcessor);
    md.with_preprocessor(AutoTitle::new());
    md.with_preprocessor(Backlinks);
    md.build().expect("Builing failed");

    Ok(())
}
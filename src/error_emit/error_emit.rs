use codespan_reporting::{
    diagnostic::{Diagnostic, Label, LabelStyle},
    files::SimpleFiles,
    term::termcolor::ColorChoice,
    term::{
        self,
        termcolor::{Buffer, StandardStream},
    },
};
use fxhash::FxHashMap;

use std::ops::Range;
pub struct ErrorReporter<'a> {
    files: SimpleFiles<&'a str, String>,
    file_name_id_map: FxHashMap<String, usize>,
    diagnostic: Diagnostic<usize>,
}

impl<'a> ErrorReporter<'a> {
    pub fn new() -> ErrorReporter<'a> {
        ErrorReporter {
            files: SimpleFiles::new(),
            diagnostic: Diagnostic::error(),
            file_name_id_map: FxHashMap::default(),
        }
    }

    pub fn add_file(&mut self, name: &'a str, source: String) -> usize {
        let id = self.files.add(name, source);
        self.file_name_id_map.insert(name.to_string(), id);
        id
    }

    pub fn add_diagnostic(
        &mut self,
        file_name: &str,
        range: impl Into<Range<usize>>,
        message: String,
    ) {
        if let Some(file_id) = self.file_name_id_map.get(file_name) {
            self.diagnostic
                .labels
                .push(Label::new(LabelStyle::Primary, *file_id, range).with_message(message));
        }
    }

    pub fn pop_diagnostic(&mut self, file_name: &str) {
        if let Some(_) = self.file_name_id_map.get(file_name) {
            self.diagnostic.labels.pop();
        }
    }
    pub fn emit_std(&mut self) -> Result<(), std::io::Error> {
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();
        term::emit(&mut writer.lock(), &config, &self.files, &self.diagnostic)?;
        Ok(())
    }

    pub fn emit_string(&self) -> String {
        let mut writer = Buffer::no_color();
        let config = codespan_reporting::term::Config::default();

        term::emit(&mut writer, &config, &self.files, &self.diagnostic).unwrap();
        // println!("{}", std::str::from_utf8(writer.as_slice()).unwrap());
        format!("{}", std::str::from_utf8(writer.as_slice()).unwrap())
    }
}

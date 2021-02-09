extern crate fmtlog;

use crate::QResult;
use fmtlog::{Config, LevelFilter, formats};
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct LogBuilder {
    silent: bool,
    quiet: bool,
    verbose: bool,
    debug: bool,
    log: Option<PathBuf>,
}

impl LogBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn silent(&mut self, silent: bool) -> &mut Self {
        self.silent = silent;
        self
    }

    pub fn quiet(&mut self, quiet: bool) -> &mut Self {
        self.quiet = quiet;
        self
    }

    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose = verbose;
        self
    }

    pub fn debug(&mut self, debug: bool) -> &mut Self {
        self.debug = debug;
        self
    }

    pub fn log<T: Into<PathBuf>>(&mut self, log: Option<T>) -> &mut Self {
        self.log = log.map(|v| v.into());
        self
    }

    pub fn init(&self) -> QResult<()> {
        let mut conf = Config::new()
            .level(self.level())
            .format(self.format());

        if let Some(ref path) = self.log {
            conf = conf.output(path);
        }

        fmtlog::new(conf).set()?;
        Ok(())
    }

    fn format(&self) -> &'static str {
        match self.level() {
            LevelFilter::Off => "",
            LevelFilter::Error | LevelFilter::Warn => formats::SIMPLE1,
            LevelFilter::Info => formats::DETAIL1,
            LevelFilter::Debug | LevelFilter::Trace => formats::DEBUG1,
        }
    }

    fn level(&self) -> fmtlog::LevelFilter {
        if self.silent {
            LevelFilter::Off
        } else if self.quiet {
            LevelFilter::Error
        } else if !self.verbose && !self.debug {
            LevelFilter::Warn
        } else if self.verbose && !self.debug {
            LevelFilter::Info
        } else if !self.verbose && self.debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Trace
        }
    }

}

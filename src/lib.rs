use std::fs::Metadata;

pub mod configuration;
pub mod routes;
pub mod startup;

pub trait Log: Sync + Send {
    fn enabled(&self, metadata: &Metadata) -> bool;

   // fn log(&self, record: &Record);

    fn flush(&self);
}

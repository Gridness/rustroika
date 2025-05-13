use crate::cli::args::MainCommand;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Clap Error: {0}")]
    ClapError(#[from] clap::error::Error),

    #[error("Evalexpr Error: {0}")]
    EvalexprError(#[from] evalexpr::error::EvalexprError),

    #[error("CSV Error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Deserialization Error: {0}")]
    SerdeDeserializationError(#[from] serde::Deserializer<'_>::Error),

    #[error("Serde Json Error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Serde Yaml Error: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error("Xlsx Error: {0}")]
    XlsxError(#[from] rust_xlsxwriter::XlsxError),

    #[error("Anyhow Error: {0}")]
    AnyhowError(#[from] anyhow::Error),

    #[error("CLI Error: Interactive mode required to run '{0}' command")]
    InteractiveModeRequired(MainCommand),

    #[error("No command provided")]
    NoCommandProvided,
}

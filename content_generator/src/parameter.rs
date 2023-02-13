use crate::constant::CONTENT_GENERATOR;
use blarg::{CommandLineParser, Parameter, Scalar, Switch};
use std::path::PathBuf;

const DEFAULT_ASSET_PATH: &str = "./content_generator/assets";
const DEFAULT_CONTENT_PATH: &str = "./content_generator/content";

#[derive(Debug)]
pub(crate) struct Parameters {
    pub verbose: bool,
    pub content_path: PathBuf,
    pub asset_path: PathBuf,
    pub target_path: PathBuf,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            verbose: false,
            content_path: PathBuf::from(DEFAULT_CONTENT_PATH),
            asset_path: PathBuf::from(DEFAULT_ASSET_PATH),
            target_path: PathBuf::default(),
        }
    }
}

pub(crate) fn parse() -> Parameters {
    let mut parameters = Parameters::default();

    let clp = CommandLineParser::new(CONTENT_GENERATOR);
    let parser = clp
        .add(Parameter::option(
            Switch::new(&mut parameters.verbose, true),
            "verbose",
            Some('v'),
        ))
        .add(Parameter::option(
            Scalar::new(&mut parameters.asset_path),
            "asset-path",
            None,
        ))
        .add(Parameter::option(
            Scalar::new(&mut parameters.content_path),
            "content-path",
            None,
        ))
        .add(Parameter::argument(
            Scalar::new(&mut parameters.target_path),
            "target_path",
        ))
        .build()
        .expect("Invalid argument parser configuration");
    parser.parse();

    parameters
}

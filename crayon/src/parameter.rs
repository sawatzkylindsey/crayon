use crate::constant::CRAYON;
use blarg::{CommandLineParser, Parameter, Scalar, Switch};
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct Parameters {
    pub verbose: bool,
    pub port: u16,
    pub resource_root: PathBuf,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            verbose: false,
            port: 80,
            resource_root: PathBuf::default(),
        }
    }
}

pub(crate) fn parse() -> Parameters {
    let mut parameters = Parameters::default();

    let clp = CommandLineParser::new(CRAYON);
    let parser = clp
        .add(Parameter::option(
            Switch::new(&mut parameters.verbose, true),
            "verbose",
            Some('v'),
        ))
        .add(Parameter::argument(
            Scalar::new(&mut parameters.port),
            "port",
        ))
        .add(Parameter::argument(
            Scalar::new(&mut parameters.resource_root),
            "resource_root",
        ))
        .build()
        .expect("Invalid argument parser configuration");
    parser.parse();

    if !parameters.resource_root.is_absolute() {
        let cwd = std::env::current_dir().expect("Cannot find current directory.");
        parameters.resource_root = cwd.join(parameters.resource_root);
    }

    if parameters.resource_root.is_dir() {
        parameters
    } else {
        panic!("Invalid resource_root: {:?}", parameters.resource_root);
    }
}

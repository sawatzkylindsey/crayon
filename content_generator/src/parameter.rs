use crate::constant::CONTENT_GENERATOR;
use blarg::{CommandLineParser, Parameter, Switch};

#[derive(Debug)]
pub(crate) struct Parameters {
    pub verbose: bool,
}

impl Default for Parameters {
    fn default() -> Self {
        Self { verbose: false }
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
        .build()
        .expect("Invalid argument parser configuration");
    parser.parse();

    parameters
}

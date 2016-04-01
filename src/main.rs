#[macro_use]
extern crate log;
extern crate log4rs;

static CONFIG: &'static str = "
appenders:
  main:
    kind: console

root:
  level: warn
  appenders:
    - main

loggers:
  log4rs_issue:
    level: trace
  log4rs_issue::nested:
    level: trace
";


mod nested {
    pub fn calls_trace() {
        trace!("calls_trace");
    }
}

fn main() {
    // Init using default, stdout logging.
    let creator = Default::default();
    let config = log4rs::file::Config::parse(CONFIG, ::log4rs::file::Format::Yaml, &creator)
                                      .expect("default config is valid")
                                      .into_config();

    log4rs::init_config(config).unwrap();

    debug!("main");
    nested::calls_trace();
}

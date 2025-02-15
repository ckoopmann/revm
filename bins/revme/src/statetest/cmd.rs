use std::path::PathBuf;

use super::runner::{find_all_json_tests, run, TestError};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cmd {
    #[structopt(required = true)]
    path: Vec<PathBuf>,
    #[structopt(short = "s", long)]
    single_thread: bool,
}

impl Cmd {
    pub fn run(&self) -> Result<(), TestError> {
        for path in &self.path {
            println!("Start running tests on: {path:?}");
            let test_files = find_all_json_tests(path);
            run(test_files, self.single_thread)?
        }
        Ok(())
    }
}

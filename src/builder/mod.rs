
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum BuildType {
    Vm,
    Iso,
    Script,
}

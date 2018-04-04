#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum Command {
    ConfigCheck,
    Scan,
    None,
}

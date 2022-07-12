
#[derive(Clone)]
pub struct Flag {
    /// The identifier of a flag
    pub(crate) id: String,
    /// The shorthands of a flag
    pub(crate) shorthands: Vec<String>,
    /// If the flag has arguments
    pub(crate) has_arguments: bool
}

impl Flag {

    /// Creates a new flag that can be used in the system
    /// for providing data
    pub fn new(id: String, shorthands: Vec<String>, has_arguments: bool) -> Self {
        Flag {id, shorthands, has_arguments}
    }
}
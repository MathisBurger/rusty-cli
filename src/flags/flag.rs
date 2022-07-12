
pub struct Flag {
    /// The identifier of a flag
    id: String,
    /// The shorthands of a flag
    shorthands: Vec<String>,
    /// If the flag has arguments
    has_arguments: bool
}

impl Flag {

    /// Creates a new flag that can be used in the system
    /// for providing data
    pub fn new(id: String, shorthands: Vec<String>, has_arguments: bool) -> Self {
        Flag {id, shorthands, has_arguments}
    }
}
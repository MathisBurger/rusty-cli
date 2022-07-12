
#[derive(Clone)]
pub struct ApplicationMetaData {
    /// The title of the application
    pub title: String,
    /// The description of the application
    pub description: String
}

impl ApplicationMetaData {

    /// Creates new project meta data that is used in the project
    /// globally for many different things
    pub fn new(title: String, description: String) -> Self {
        ApplicationMetaData {
            title,
            description
        }
    }
}
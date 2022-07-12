use crate::meta_data::ApplicationMetaData;

/// Clones the meta data of the application
pub fn clone_meta_data_option(meta_data: &Option<ApplicationMetaData>) -> Option<ApplicationMetaData> {
    return match meta_data {
        Some(x) => Some(ApplicationMetaData{title: x.title.clone(), description: x.description.clone()}),
        None => None
    }
}
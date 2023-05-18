use serde::{Serialize, Deserialize};

/// Struct containing the base record data from a store
///
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BaseItem {
    #[serde(rename = "pk")]
    partition_key: String,
    registry_id: String,
    #[serde(rename = "")]
    repository_id: String,
    document_type: String,
    repository: String,
    version: String,
    created_at: String,
    deleted_at: String,
    #[serde(default)]
    no_change: bool,
    #[serde(default)]
    no_delete: bool,
    #[serde(default)]
    no_list: bool,
    #[serde(default)]
    no_read: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    annotation_artifact_created: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    annotation_oci_artifact_created: String,
    #[serde(default)]
    state: ManagementState,
}

impl BaseItem {
    /// Returns a tuple of strings that identify a row,
    ///
    pub fn get_row_id(&self) -> (&String, &String) {
        (&self.partition_key, &self.registry_id)
    }

    /// Sets the partition and registry id
    /// 
    pub fn set_row_id(&mut self, partition_key: impl Into<String>, registry_id: impl Into<String>) {
        self.partition_key = partition_key.into();
        self.registry_id = registry_id.into();
    }
}

/// Enumeration over specific management states
///
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub enum ManagementState {
    #[default]
    Default,
    Active,
    DeleteInitiated,
    Deleted,
    SoftDeleteInitiated,
    SoftDeleted,
    Pending,
}

/// Super-trait representing an item,
/// 
pub trait Item<'a> : Clone + AsMut<BaseItem> + AsRef<BaseItem> + Serialize + Deserialize<'a> 
{
    /// Returns the partion as a str
    /// 
    fn partition(&self) -> &str;

    /// Returns the id as a str 
    /// 
    fn id(&self) -> &str;
}
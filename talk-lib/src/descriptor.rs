use std::collections::BTreeMap;
use serde::Serialize;
use serde::Deserialize;
use crate::Item;
use crate::BaseItem;

/// Struct that contains fields for an OCI object descriptor
///
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Descriptor {
    /// Digest this descriptor represents
    /// 
    digest: String,
    /// Media-type this descriptor represents
    /// 
    #[serde(rename = "mediaType")]
    media_type: String,
    /// Artifact-type this descriptor represents
    /// 
    #[serde(rename = "artifactType")]
    artifact_type: String,
    /// Size in bytes of the content this descriptor represents
    /// 
    size: usize,
    /// Ordered map of annotations of this image
    /// 
    annotations: BTreeMap<String, String>,
    /// Base item used to organize and index this descriptor
    /// 
    #[serde(skip)]
    _base_item: BaseItem,
}

impl Descriptor {
    /// Creates a new descriptor
    /// 
    pub fn new(
        digest: impl Into<String>,
        media_type: impl Into<String>,
        size: usize,
        annotations: BTreeMap<String, String>,
    ) -> Self {
        let media_type = media_type.into();
        Self {
            digest: digest.into(),
            media_type: media_type.clone(),
            artifact_type: media_type,
            size,
            annotations,
            _base_item: BaseItem::default(),
        }
    }

    /// Sets the current digest
    /// 
    pub fn set_digest(&mut self, digest: impl Into<String>) {
        self.digest = digest.into();
    }
}

impl AsRef<BaseItem> for Descriptor {
    fn as_ref(&self) -> &BaseItem {
        &self._base_item
    }
}

impl AsMut<BaseItem> for Descriptor {
    fn as_mut(&mut self) -> &mut BaseItem {
        &mut self._base_item
    }
}

impl<'a> Item<'a> for Descriptor 
{
    fn partition(&self) -> &str {
        self.media_type.as_str()
    }

    fn id(&self) -> &str {
        self.digest.as_str()
    }
}

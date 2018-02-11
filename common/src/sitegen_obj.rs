
use gamedata::site::SiteKind;

/// Hold data for site generation
#[derive(Clone, Serialize, Deserialize)]
pub struct SiteGenObject {
    pub id: String,
    pub kind: SiteKind,
    pub map_template_id: Vec<String>
}



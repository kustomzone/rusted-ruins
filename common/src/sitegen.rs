use crate::gamedata::faction::FactionId;
use crate::gamedata::map::SiteSymbolKind;
use crate::gamedata::shop::ShopKind;
use crate::gamedata::site::SiteKind;
use geom::Vec2d;

/// Hold data for site generation
#[derive(Clone, Serialize, Deserialize)]
pub struct SiteGenObject {
    pub id: String,
    pub kind: SiteKind,
    pub site_symbol: SiteSymbolKind,
    pub default_faction_id: FactionId,
    pub map_template_id: Vec<String>,
    pub unique_citizens: Vec<UniqueCitizenGenData>,
    pub shops: Vec<ShopGenData>,
}

/// Data to generate a unique citizen
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct UniqueCitizenGenData {
    pub pos: Vec2d,
    pub floor: u32,
    pub name: Option<String>,
    /// Unique number in this site
    pub n: u32,
    pub chara_template_id: String,
    pub talk_script_id: Option<String>,
}

/// Data to generate a shop on the site
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ShopGenData {
    /// Shopkeeper's id (n)
    pub chara_n: u32,
    pub kind: ShopKind,
    #[serde(default)]
    pub id: Vec<String>,
}

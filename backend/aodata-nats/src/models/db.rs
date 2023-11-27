#[derive(serde::Deserialize)]
pub struct Localization {
    #[serde(rename = "LocalizedNames")]
    pub localized_names: Option<LocalizedValues>,

    #[serde(rename = "LocalizedDescriptions")]
    pub localized_descriptions: Option<LocalizedValues>,

    #[serde(rename = "UniqueName")]
    pub item: String,
}

#[derive(serde::Deserialize)]
pub struct LocalizedValues {
    #[serde(rename = "EN-US")]
    pub en_us: String,
    #[serde(rename = "DE-DE")]
    pub de_de: String,
    #[serde(rename = "FR-FR")]
    pub fr_fr: String,
    #[serde(rename = "RU-RU")]
    pub ru_ru: String,
    #[serde(rename = "PL-PL")]
    pub pl_pl: String,
    #[serde(rename = "ES-ES")]
    pub es_es: String,
    #[serde(rename = "PT-BR")]
    pub pt_br: String,
    #[serde(rename = "IT-IT")]
    pub it_it: String,
    #[serde(rename = "ZH-CN")]
    pub zh_cn: String,
    #[serde(rename = "KO-KR")]
    pub ko_kr: String,
    #[serde(rename = "JA-JP")]
    pub ja_jp: String,
    #[serde(rename = "ZH-TW")]
    pub zh_tw: String,
    #[serde(rename = "ID-ID")]
    pub id_id: String,
}

#[derive(serde::Deserialize)]
pub struct Location {
    #[serde(rename = "Index")]
    pub id: String,
    #[serde(rename = "UniqueName")]
    pub name: String,
}

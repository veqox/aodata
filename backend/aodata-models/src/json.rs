#[derive(serde::Deserialize)]
pub struct Localization {
    #[serde(rename = "LocalizedNames")]
    pub localized_names: Option<LocalizedName>,
    #[serde(rename = "LocalizedDescriptions")]
    pub localized_descriptions: Option<LocalizedDescription>,
    #[serde(rename = "UniqueName")]
    pub item: String,
    #[serde(rename = "Index")]
    pub id: String,
}

#[derive(serde::Deserialize)]
pub struct LocalizedName {
    #[serde(rename = "EN-US")]
    pub en_us: Option<String>,
    #[serde(rename = "DE-DE")]
    pub de_de: Option<String>,
    #[serde(rename = "FR-FR")]
    pub fr_fr: Option<String>,
    #[serde(rename = "RU-RU")]
    pub ru_ru: Option<String>,
    #[serde(rename = "PL-PL")]
    pub pl_pl: Option<String>,
    #[serde(rename = "ES-ES")]
    pub es_es: Option<String>,
    #[serde(rename = "PT-BR")]
    pub pt_br: Option<String>,
    #[serde(rename = "IT-IT")]
    pub it_it: Option<String>,
    #[serde(rename = "ZH-CN")]
    pub zh_cn: Option<String>,
    #[serde(rename = "KO-KR")]
    pub ko_kr: Option<String>,
    #[serde(rename = "JA-JP")]
    pub ja_jp: Option<String>,
    #[serde(rename = "ZH-TW")]
    pub zh_tw: Option<String>,
    #[serde(rename = "ID-ID")]
    pub id_id: Option<String>,
    #[serde(rename = "TR-TR")]
    pub tr_tr: Option<String>,
    #[serde(rename = "AR-SA")]
    pub ar_sa: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct LocalizedDescription {
    #[serde(rename = "EN-US")]
    pub en_us: Option<String>,
    #[serde(rename = "DE-DE")]
    pub de_de: Option<String>,
    #[serde(rename = "FR-FR")]
    pub fr_fr: Option<String>,
    #[serde(rename = "RU-RU")]
    pub ru_ru: Option<String>,
    #[serde(rename = "PL-PL")]
    pub pl_pl: Option<String>,
    #[serde(rename = "ES-ES")]
    pub es_es: Option<String>,
    #[serde(rename = "PT-BR")]
    pub pt_br: Option<String>,
    #[serde(rename = "IT-IT")]
    pub it_it: Option<String>,
    #[serde(rename = "ZH-CN")]
    pub zh_cn: Option<String>,
    #[serde(rename = "KO-KR")]
    pub ko_kr: Option<String>,
    #[serde(rename = "JA-JP")]
    pub ja_jp: Option<String>,
    #[serde(rename = "ZH-TW")]
    pub zh_tw: Option<String>,
    #[serde(rename = "ID-ID")]
    pub id_id: Option<String>,
    #[serde(rename = "TR-TR")]
    pub tr_tr: Option<String>,
    #[serde(rename = "AR-SA")]
    pub ar_sa: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct Location {
    #[serde(rename = "Index")]
    pub id: String,
    #[serde(rename = "UniqueName")]
    pub name: String,
}

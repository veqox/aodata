use aodata_models::json;
use sqlx::PgPool;

pub async fn insert_locations(
    pool: &PgPool,
    locations: Vec<json::Location>,
) -> Result<(), sqlx::Error> {
    let transaction = pool.begin().await.unwrap();

    for location in locations {
        sqlx::query!(
            "
INSERT INTO location (
    id,
    name)
VALUES (
    $1,
    $2)
ON CONFLICT DO
    NOTHING",
            location.id,
            location.name
        )
        .execute(pool)
        .await
        .unwrap();
    }

    transaction.commit().await.unwrap();

    Ok(())
}

pub async fn insert_localizations(
    pool: &PgPool,
    localizations: Vec<json::Localization>,
) -> Result<(), sqlx::Error> {
    let transaction = pool.begin().await?;

    for localization in localizations {
        let item_id: i32 = match localization.id.parse() {
            Ok(id) => id, // should be fine since item ids go to 10572 as of 2024-30-01
            Err(_) => continue,
        };

        sqlx::query!(
            "
INSERT INTO item (
    id,
    unique_name)
VALUES (
    $1,
    $2)
ON CONFLICT DO
    NOTHING",
            item_id,
            localization.item
        )
        .execute(pool)
        .await?;

        let localized_names = match localization.localized_names {
            Some(localized_names) => localized_names,
            None => continue,
        };

        sqlx::query!(
            "
INSERT INTO localized_name (
    item_unique_name,
    en_us,
    de_de,
    fr_fr,
    ru_ru,
    pl_pl,
    es_es,
    pt_br,
    it_it,
    zh_cn,
    ko_kr,
    ja_jp,
    zh_tw,
    id_id,
    tr_tr,
    ar_sa)
VALUES (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7,
        $8,
        $9,
        $10,
        $11,
        $12,
        $13,
        $14,
        $15,
        $16)
ON CONFLICT(item_unique_name) DO
    UPDATE SET
        en_us = $2,
        de_de = $3,
        fr_fr = $4,
        ru_ru = $5,
        pl_pl = $6,
        es_es = $7,
        pt_br = $8,
        it_it = $9,
        zh_cn = $10,
        ko_kr = $11,
        ja_jp = $12,
        zh_tw = $13,
        id_id = $14,
        tr_tr = $15,
        ar_sa = $16",
            localization.item,
            localized_names.en_us,
            localized_names.de_de,
            localized_names.fr_fr,
            localized_names.ru_ru,
            localized_names.pl_pl,
            localized_names.es_es,
            localized_names.pt_br,
            localized_names.it_it,
            localized_names.zh_cn,
            localized_names.ko_kr,
            localized_names.ja_jp,
            localized_names.zh_tw,
            localized_names.id_id,
            localized_names.tr_tr,
            localized_names.ar_sa,
        )
        .execute(pool)
        .await?;

        let localized_descriptions = match localization.localized_descriptions {
            Some(localized_descriptions) => localized_descriptions,
            None => continue,
        };

        sqlx::query!(
            "
INSERT INTO localized_description (
    item_unique_name,
    en_us,
    de_de,
    fr_fr,
    ru_ru,
    pl_pl,
    es_es,
    pt_br,
    it_it,
    zh_cn,
    ko_kr,
    ja_jp,
    zh_tw,
    id_id,
    tr_tr,
    ar_sa)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9,
    $10,
    $11,
    $12,
    $13,
    $14,
    $15,
    $16)
ON CONFLICT (item_unique_name) DO
    UPDATE SET
        en_us = $2,
        de_de = $3,
        fr_fr = $4,
        ru_ru = $5,
        pl_pl = $6,
        es_es = $7,
        pt_br = $8,
        it_it = $9,
        zh_cn = $10,
        ko_kr = $11,
        ja_jp = $12,
        zh_tw = $13,
        id_id = $14,
        tr_tr = $15,
        ar_sa = $16",
            localization.item,
            localized_descriptions.en_us,
            localized_descriptions.de_de,
            localized_descriptions.fr_fr,
            localized_descriptions.ru_ru,
            localized_descriptions.pl_pl,
            localized_descriptions.es_es,
            localized_descriptions.pt_br,
            localized_descriptions.it_it,
            localized_descriptions.zh_cn,
            localized_descriptions.ko_kr,
            localized_descriptions.ja_jp,
            localized_descriptions.zh_tw,
            localized_descriptions.id_id,
            localized_descriptions.tr_tr,
            localized_descriptions.ar_sa,
        )
        .execute(pool)
        .await?;
    }

    transaction.commit().await?;

    Ok(())
}

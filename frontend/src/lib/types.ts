export interface MarketOrderCountByItem {
    item_unique_name: string;
    count: number;
}

export interface MarketOrderCountByLocation {
    location: string;
    count: number;
}

export interface MarketOrderCountByAuctionType {
    auction_type: string;
    count: number;
}

export interface MarketOrderCountByUpdatedAt {
    updated_at: Date;
    count: number;
}

export interface MarketOrderCountByCreatedAt {
    created_at: Date;
    count: number;
}

export interface MarketOrderCount {
    count: number;
}

export interface LocalizedNames {
    item_unique_name: string,
    en_us: string,
    de_de: string,
    fr_fr: string,
    ru_ru: string,
    pl_pl: string,
    es_es: string,
    pt_br: string,
    it_it: string,
    zh_cn: string,
    ko_kr: string,
    ja_jp: string,
    zh_tw: string,
    id_id: string,
}

export interface MarketOrder {
    location: string,
    quality_level: number,
    enchantment_level: number,
    unit_price_silver: number,
    amount: number,
    auction_type: string,
    expires_at: Date,
    updated_at: Date,
}
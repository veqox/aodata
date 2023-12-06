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

export interface MarketOrderCount {
    count: number;
}
import type { MakretOrderCountByUpdatedAt, MarketOrderCount, MarketOrderCountByAuctionType, MarketOrderCountByItem, MarketOrderCountByLocation, } from '$lib/types';
import type { PageServerLoad } from './$types'

export const load: PageServerLoad = async ({fetch}) => {
    let market_order_count_by_item_response = await fetch("http://aodata-api:8080/statistics/orders/item");
    let market_order_count_by_location_response = await fetch("http://aodata-api:8080/statistics/orders/location");
    let market_order_count_by_auction_type_response = await fetch("http://aodata-api:8080/statistics/orders/auction_type");
    let market_order_count_by_updated_at_response = await fetch("http://aodata-api:8080/statistics/orders/hourly");
    let market_order_count_response = await fetch("http://aodata-api:8080/statistics/orders/count");

    let market_order_count_by_item = await market_order_count_by_item_response.json() as MarketOrderCountByItem[];
    let market_order_count_by_location = await market_order_count_by_location_response.json() as MarketOrderCountByLocation[];
    let market_order_count_by_auction_type = await market_order_count_by_auction_type_response.json() as MarketOrderCountByAuctionType[];
    let market_order_count_by_updated_at = await market_order_count_by_updated_at_response.json() as MakretOrderCountByUpdatedAt[];
    let market_order_count = await market_order_count_response.json() as MarketOrderCount;

    return {
        props: {
            data: {
                market_order_count,
                market_order_count_by_item,
                market_order_count_by_location,
                market_order_count_by_auction_type,
                market_order_count_by_updated_at
            
            }
        }
    }
}
import type { MarketOrderCountByUpdatedAt, MarketOrderCount, MarketOrderCountByAuctionType, MarketOrderCountByLocation, MarketOrderCountByCreatedAt, MarketOrderCountByUpdatedAtAndLocation, } from '$lib/types';
import type { PageServerLoad } from './$types'

export const load: PageServerLoad = async ({fetch}) => {
    let market_order_count_by_location_response = await fetch("http://localhost:8080/api/statistics/orders?group_by=location");
    let market_order_count_by_auction_type_response = await fetch("http://localhost:8080/api/statistics/orders?group_by=auction_type");
    let market_order_count_by_updated_at_response = await fetch("http://localhost:8080/api/statistics/orders?group_by=updated_at");
    let market_order_count_by_updated_at_and_location_response = await fetch("http://localhost:8080/api/statistics/orders?group_by=updated_at,location");
    let market_order_count_by_created_at_response = await fetch("http://localhost:8080/api/statistics/orders?group_by=created_at");
    let market_order_count_response = await fetch("http://localhost:8080/api/statistics/orders/count");

    let market_order_count_by_location = await market_order_count_by_location_response.json() as MarketOrderCountByLocation[];
    let market_order_count_by_auction_type = await market_order_count_by_auction_type_response.json() as MarketOrderCountByAuctionType[];
    let market_order_count_by_updated_at = await market_order_count_by_updated_at_response.json() as MarketOrderCountByUpdatedAt[];
    let market_order_count_by_created_at = await market_order_count_by_created_at_response.json() as MarketOrderCountByCreatedAt[]
    let market_order_count_by_updated_at_and_location = await market_order_count_by_updated_at_and_location_response.json() as MarketOrderCountByUpdatedAtAndLocation[];
    let market_order_count = await market_order_count_response.json() as MarketOrderCount;

    return {
        props: {
            data: {
                market_order_count,
                market_order_count_by_location,
                market_order_count_by_auction_type,
                market_order_count_by_updated_at,
                market_order_count_by_created_at,
                market_order_count_by_updated_at_and_location,
            }
        }
    }
}
import type { MarketOrderCount } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
  let market_order_count_request_response = await fetch(
    "http://aodata-api:8080/api/statistics/orders/count?auction_type=request",
  );
  let market_order_count_offer_response = await fetch(
    "http://aodata-api:8080/api/statistics/orders/count?auction_type=offer",
  );
  let market_order_count_response = await fetch(
    "http://aodata-api:8080/api/statistics/orders/count",
  );

  let market_order_count =
    (await market_order_count_response.json()) as MarketOrderCount;
  let market_order_count_request =
    (await market_order_count_request_response.json()) as MarketOrderCount;
  let market_order_count_offer =
    (await market_order_count_offer_response.json()) as MarketOrderCount;

  return {
    props: {
      data: {
        market_order_count,
        market_order_count_request,
        market_order_count_offer,
      },
    },
  };
};

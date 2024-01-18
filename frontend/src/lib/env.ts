import { PUBLIC_DEV_BACKEND_URL, PUBLIC_PROD_BACKEND_URL } from "$env/static/public"

export const get_backend_url = () => {
    if (process.env.ENV === "PROD") {
        return PUBLIC_PROD_BACKEND_URL;
    }
    
    return PUBLIC_DEV_BACKEND_URL;
}
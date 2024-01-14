import type { PageLoad } from "./$types";

export const load: PageLoad = ({ params }) => {
  return {
    props: {
      slug: params.slug,
    },
  };
};

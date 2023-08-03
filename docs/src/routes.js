import { wrap } from "svelte-spa-router/wrap";

// Components: only Home, Loading and NotFound are statically included in the bundle
import Home from "./lib/routes/Home.svelte";
import Loading from "./lib/routes/Loading.svelte";
import Four0Four from "./lib/routes/Four0Four.svelte";
import Documentation from "./lib/routes/Documentation.svelte";

export default {
  // Exact path
  "/": Home,

  "/documentation":Documentation,

  "/hello/:first/:last?": wrap({
    asyncComponent: () => import("./lib/routes/Name.svelte"),
    //passed only to the route events
    userData: { hello: "world" },

    props: {
      num: 42,
    },

    loadingComponent: Loading,
    loadingParams: {
      message: "Loading the Name route…",
    },

    conditions: [
      (detail) => {
        // Pre-condition succeeds only 50% of times
        return Math.random() > 0.5;
      },
      // Second pre-condition function (async)
      async (detail) => {
        console.log(detail.location); //check git for the fields()
        return true;
      },
    ],
  }),

  // Wildcard parameter
  // This matches `/wild/*` (with anything after), but NOT `/wild` (with nothing after)
  // This is dynamically imported too
  "/wild/*": wrap({
    asyncComponent: () =>
      import("./lib/routes/Wild.svelte").then((component) => {
        return new Promise((resolve) => {
          // Wait 5 seconds before returning
          setTimeout(() => resolve(component), 5000);
        });
      }),

    loadingComponent: Loading,
    loadingParams: {
      message: "Loading the Wild route…",
    },
  }),
  "/denied": Four0Four,
  // Catch-all, must be last
  "*": Four0Four,
};

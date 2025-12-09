// Disable SSR for all routes - render everything client-side
// This is needed because we fetch data from an external API
// that isn't available during SSR
export const ssr = false;

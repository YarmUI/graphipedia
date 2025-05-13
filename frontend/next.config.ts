import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  async rewrites() {
    if (process.env.NODE_ENV === "development") {
      return [
        {
          source: '/api/search',
          destination: 'http://localhost:3000/api/search',
        },
      ];
    }
    return [];
  },
};

export default nextConfig;

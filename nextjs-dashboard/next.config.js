/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  images: {
      unoptimized: true
  },
  trailingSlash: true,
  distDir: '../shuttle/static'
};
module.exports = nextConfig;

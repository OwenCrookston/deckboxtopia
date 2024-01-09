/** @type {import('next').NextConfig} */
const { PHASE_PRODUCTION_BUILD } = require('next/constants');

const nextBuildConfig = {
  output: 'export',
  images: {
      unoptimized: true
  },
  trailingSlash: true,
  distDir: '../shuttle/static',
  publicRuntimeConfig: {
    apiBaseUrl: '',
  },
};

const nextDevConfig = {
  images: {
      unoptimized: true
  },
  trailingSlash: true
};

module.exports = (phase, { defaultConfig }) => {
    if (phase === PHASE_PRODUCTION_BUILD) {
        return nextBuildConfig;
    }
    return nextDevConfig;
};

// next.config.js
const { i18n } = require("./next-i18next.config");

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  i18n, // Reuse the same i18n config
};

module.exports = nextConfig;

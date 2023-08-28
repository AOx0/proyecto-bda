/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs", "./templates/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [],
  safelist: [
    {
      pattern: /bg-+/, // This includes bg of all colors and shades
    },
  ],
}


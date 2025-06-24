module.exports = {
  mode: "jit",
  content: {
    files: ["src/**/*.rs", "index.html"],
  },
  darkMode: "media", // 'media' or 'class'
  theme: {
    extend: {
      fontFamily: {
        public: ['"Public Sans"', "sans-serif"],
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};

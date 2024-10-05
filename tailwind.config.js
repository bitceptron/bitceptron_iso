/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        first_color: "#9E6649",
        second_color: "#DDCEB3",
        third_color: "#FFA66A",
        disabled_area_color: "#C3B091",
        button_color_enabled:"#DDCEB3",
        button_color_disabled: "#d5dbc4",
        button_color_hover:"#f7931a",
        button_color_action:"#FFBB00",
      },
    },
  },
  plugins: [],
};

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte}'],
  theme: {
    extend: {
      colors: {
        greenTealinux: "#26A768",
        greyButton: "#929AAB",
        greyBorder: "#E4E7E6",
        greyVariant: "#D6D6D6",
        userCheckBox: "#757575",
        grayTealinux: "#E7EDED",
        whiteTealinux: "#FFFEFB",
      },
      fontFamily: {
        poppin: ['poppins', 'sans-serif'],
        poppinsemibold: ['poppinssemibold', 'sans-serif'],
        poppinmedium: ['poppinmedium', 'sans-serif'],
        archivo: ['archivo', 'sans-serif'],
      }
    },
  },
  plugins: [],
}


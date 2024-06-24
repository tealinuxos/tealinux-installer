/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte}'],
  theme: {
    extend: {
      colors: {
        greenTealinux: "#26A768",
      },
      fontFamily: {
        poppin: ['poppins', 'sans-serif'],
        poppinsemibold: ['poppinssemibold', 'sans-serif'],
        archivo: ['archivo', 'sans-serif'],
      }
    },
  },
  plugins: [],
}


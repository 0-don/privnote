/** @type {import('tailwindcss').Config}*/
const config = {
  content: ['./src/**/*.{html,js,svelte,ts}'],

  theme: {
    extend: {
      colors: {
        body: {
          DEFAULT: '#242729'
        },
        main: {
          DEFAULT: '#7D4698'
        },
        alt: {
          DEFAULT: '#4E5457'
        },
        container: {
          DEFAULT: '#333639'
        }
      }
    },
    container: {
      center: true,
      padding: '1rem',
      screens: {
        sm: '600px',
        md: '728px',
        lg: '984px'
      }
    }
  },

  plugins: [require('@tailwindcss/forms')]
};

module.exports = config;

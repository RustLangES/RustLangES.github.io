/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    fontFamily: {
      "alfa-slab": ["Alfa Slab One", "sans-serif"],
      "work-sans": ["Work Sans", "sans-serif"],
    },
    extend: {
      screens: (theme) => ({
        'xs': '475px',
        ...theme.screens,
      }),
      backgroundImage: (theme) => ({
        "kaku-dev": "url('/assets/kaku.avif')",
        "kaku": "url('/kaku.avif')",
      }),
      gridTemplateColumns:  (theme) => ({
        "divided": "2.5fr 1fr",
        "sidebar": "1fr 1fr"
      }),
      transitionProperty: {
        'height': 'height',
        'width': 'width',
        'size': 'height, width'
      },
      fill: (theme) => ({
        "shape-fill-light": "rgb(203 213 225 / 1)",
        "shape-fill-dark": "rgb(39 39 42 / 1)",
      }),
      colors: {
        'serenade': {
          '50': '#fff3e5',
          '100': '#ffecd5',
          '200': '#fed6aa',
          '300': '#fdb874',
          '400': '#fb8f3c',
          '500': '#f96f16',
          '600': '#ea540c',
          '700': '#c23e0c',
          '800': '#9a3212',
          '900': '#7c2b12',
          '950': '#431307',
        },
        'clementine': {
          '50': 'oklch(98.47% 0.02 92.50)',
          '100': 'oklch(95.64% 0.06 91.09)',
          '200': 'oklch(91.38% 0.11 90.83)',
          '300': 'oklch(86.40% 0.15 85.81)',
          '400': 'oklch(81.85% 0.17 76.80)',
          '500': 'oklch(75.16% 0.17 60.36)',
          '600': 'oklch(65.36% 0.17 49.96)',
          '700': 'oklch(54.45% 0.16 42.31)',
          '800': 'oklch(46.51% 0.14 40.45)',
          '900': 'oklch(40.62% 0.12 40.32)',
          '950': 'oklch(27.29% 0.08 40.20)',
        },
        'black_bean': {
          DEFAULT: '#2e140f', 
          100: '#090403',
          200: '#120806', 
          300: '#1c0c09', 
          400: '#25100c', 
          500: '#2e140f', 
          600: '#723225', 
          700: '#b64f3b', 
          800: '#d38777', 
          900: '#e9c3bb'
        }, 
        'orange_(pantone)': { 
          DEFAULT: '#f96817', 
          100: '#351401',
          200: '#692803',
          300: '#9e3d04',
          400: '#d35105',
          500: '#f96817',
          600: '#fa8744',
          700: '#fba573',
          800: '#fdc3a2',
          900: '#fee1d0'
        }
      }
    },
  },
  plugins: [],
};
/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: 'class',
    purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
        ],
    },
    theme: {
        extend: {
            colors: {
                'primary': '#182628',
                'secondary': '#FAF0E6',
            }
        },
    },
    plugins: [],
}
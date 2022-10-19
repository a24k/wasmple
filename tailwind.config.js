/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx,css,md,mdx,html,json,scss}",
    ],
    theme: {
        extend: {
            animation: {
                loader: 'loader 0.6s infinite alternate',
            },
            keyframes: {
                loader: {
                    to: {
                        opacity: 0.1,
                        transform: 'translate3d(0, -1rem, 0)',
                    },
                },
            },
        },
    },
    plugins: [
        require("@tailwindcss/forms"),
    ],
}

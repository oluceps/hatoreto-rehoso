/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ["./src/**/*.{js,jsx,ts,tsx}"],
	theme: {
		extend: {},
		colors: {
			"cerise-red": {
				50: "#fef2f4",
				100: "#fde6e9",
				200: "#fbd0d9",
				300: "#f7aab9",
				400: "#f27a93",
				500: "#e63f66",
				600: "#d42a5b",
				700: "#b21e4b",
				800: "#951c45",
				900: "#801b40",
				950: "#470a1f",
			},
		},
	},
	plugins: [require("daisyui")],
};

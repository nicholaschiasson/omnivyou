module.exports = {
	mode: "jit",
	purge: [
		"src/**/*.rs"
	],
	darkMode: false, // or 'media' or 'class'
	theme: {
		extend: {
			scale: {
				'400': '4',
				'500': '5',
			},
			transitionDuration: {
				'2000': '2000ms',
			}
		}
	},
	variants: {
		extend: {},
	},
	plugins: [],
}

module.exports = {
	mode: "jit",
	purge: [
		"src/**/*.rs"
	],
	darkMode: false, // or 'media' or 'class'
	theme: {
		flex: {
			'2': '2 2 0%',
		},
		scale: {
			'400': '4',
			'500': '5',
		}
	},
	variants: {
		extend: {},
	},
	plugins: [],
}

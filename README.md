# onmiview

## ToDo

### V1

- [x] Keyboard controls
	- [x] Arrows to navigate next and previous
	- [x] Escape to do the same as X button
- [x] Use official wasm-bindgen, web-sys, and js-sys
- [x] Add an X button to quit viewing and return to initial file picker page
- [x] Add settings button in viewing mode to toggle settings panel
	- [x] Content specific settings
		- [x] Audio autoplay
		- [x] Audio autoplay delay
		- [x] Image autoplay
		- [x] Image autoplay delay
		- [x] Video autoplay
		- [x] Video autoplay delay
- [x] Make it so clicking outside of menu closes the menu
- [ ] Add visual warning when no files could be found
- [ ] GitHub action to build pages folder
- [ ] Buy omnivyou.com
- [ ] Use custom domain in GitHub pages
- [ ] GitHub action for letsencrypt

### V2

- [x] Build tailwind with build step
- [ ] Build tailwind with trunk hook
- [ ] Style the initial file picker page further
- [ ] Add swipe capability for mobile
- [ ] Add transitions?
	- [ ] Fade
- [ ] Hack FileSystemAccess binding to get directory entries
- [ ] Use `showDirectoryPicker` instead of file input element
- [ ] Add directory indexing
- [ ] Add options for directory items

## Notes

- Make sure to install `wasm-bindgen-cli` from my fork with `cargo install --git https://github.com/nicholaschiasson/wasm-bindgen wasm-bindgen-cli`.

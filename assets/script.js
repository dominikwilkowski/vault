function get_os() {
	const platform = (window.navigator?.userAgentData?.platform || window.navigator.platform).toLowerCase();
	const macos_platforms = ["macos", "macintosh", "macintel", "macppc", "mac68k"];
	const windows_platforms = ["win32", "win64", "windows", "wince"];

	if (macos_platforms.indexOf(platform) !== -1) {
		return "macOS";
	} else if (windows_platforms.indexOf(platform) !== -1) {
		return "Windows";
	} else {
		return "macOS";
	}
}

let os = get_os();
let link = document.querySelector("#"+ os +"_link").getAttribute('href');
document.querySelector("#cta").setAttribute('href', link);

document.querySelector("#macOS_icon").classList.add("hidden");
document.querySelector("#Windows_icon").classList.add("hidden");
document.querySelector("#" + os + "_icon").classList.remove("hidden");


document.querySelector("#marketing-bs-btn").addEventListener('click', () => {
	let body = document.querySelector("body");
	if (body.classList.contains("marketing-bs")) {
		body.classList.remove("marketing-bs");
		body.classList.add("plain");
	} else {
		body.classList.remove("plain");
		body.classList.add("marketing-bs");
	}
});

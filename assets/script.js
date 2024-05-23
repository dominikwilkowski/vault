function getOS() {
	const userAgent = window.navigator.userAgent;
	const platform = window.navigator?.userAgentData?.platform || window.navigator.platform;
	const macosPlatforms = ['macOS', 'Macintosh', 'MacIntel', 'MacPPC', 'Mac68K'];
	const windowsPlatforms = ['Win32', 'Win64', 'Windows', 'WinCE'];

	if (macosPlatforms.indexOf(platform) !== -1) {
		return 'macOS';
	} else if (windowsPlatforms.indexOf(platform) !== -1) {
		return 'Windows';
	} else if (/Linux/.test(platform)) {
		return 'Linux';
	}

	return 'unknown';
}

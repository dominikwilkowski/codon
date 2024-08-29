function start_qr_scanner(video_element) {
	const scanner = new QrScanner(video_element, (result) => console.log(result));
	scanner.start();
}

// Expose the function globally
window.start_qr_scanner = start_qr_scanner;

# Release Notes

## v0.0.2
**Release Date:** June 1, 2025

### Overview
This release enhances `grok-v2a` with client-side webcam support, an image preview feature, and fixes for layout stability and user interface clarity. It builds on the initial release by improving user interaction and streamlining the image description process, maintaining the X-inspired GUI and robust backend.

### Features
- **Webcam Feed Processing** ([93bb84d](https://github.com/xbladestealth/grok-v2a/commit/93bb84d13237933371da7ad8f365c7fa6a06af07))  
  Added support for capturing images directly from the client's webcam:
  - Implemented a "Webcam Mode" toggle to access the camera feed.
  - Enabled image capture with a "Capture" button, storing snapshots for submission.
  - Added functionality to restart the feed and take new images.
  - Blocked form submission during active webcam mode (`buttonState === "capture"`) with an error alert for better UX.

- **Image Preview Feature** ([b74a716](https://github.com/xbladestealth/grok-v2a/commit/b74a71695acbb5a87baba9742856db9042c48c68))  
  Introduced a preview for uploaded or captured images, replacing redundant response visualization:
  - Added `<img class="preview-image">` within `.image-preview` to display selected files, captured snapshots, or the default image (`/static/images/deca_bg.jpg`).
  - Removed redundant image display in the response, focusing on markdown-rendered descriptions.
  - Included client-side validation and error handling for image loading failures.

### Fixes
- **Prevent Layout Shift with Scrollbar** ([72e1455](https://github.com/xbladestealth/grok-v2a/commit/72e14554e1d9af523e30f75729246551250ce480))  
  Fixed content shifting left during form submission:
  - Added `html { overflow-y: scroll; }` to always display the vertical scrollbar, ensuring consistent layout width.
  - Stabilized `body` and `#response-container` with fixed widths and minimum heights to prevent reflows.

- **Improved Webcam Button Label** ([139dc1f](https://github.com/xbladestealth/grok-v2a/commit/139dc1f8e04313d3636cb851c49d12cfc41c3b0d))  
  Replaced the "Recapture" button label with "Restart" for clarity:
  - Updated the button text to better reflect the action of restarting the webcam feed for a new capture.
  - Enhanced user experience by avoiding confusion with immediate re-capture implications.

### Known Issues
- An issue has been reported regarding the application’s functionality. See [Issue #3](https://github.com/xbladestealth/grok-v2a/issues/3) for details and updates.

## v0.0.1
**Release Date:** May 25, 2025

### Overview
This is the initial release of `grok-v2a`, a web application designed to describe images using the xAI API. This version sets up the foundational backend with Axum and introduces a user-friendly, X-inspired GUI.

### Features
- **Image Description Backend** ([988332a](https://github.com/xbladestealth/grok-v2a/commit/988332a7453509ecdff5f047af6be6f63e48f4b1))  
  Added an Axum web application to describe images using the xAI API. This includes:
  - A form to upload images (JPEG/PNG, max 5MB) and a prompt.
  - Integration with the xAI API for image description.
  - Client-side validation for file type and size.
  - Markdown rendering and DOMPurify sanitization for safe response display.

- **X-like GUI** ([8e66a26](https://github.com/xbladestealth/grok-v2a/commit/8e66a267c630ae50e9476709e023a26c5420c675))  
  Enhanced the user interface with an X-inspired design for better user experience:
  - Added a clean, modern GUI with gray input boxes, pill-shaped buttons, and no outline on the "Choose Image" button.
  - Implemented a dark mode toggle with theme-aware logos.
  - Added clickable X logo linking to the project’s X page (`https://x.com/xbladestealth`).

### Known Issues
- An issue has been reported regarding the application’s functionality. See [Issue #3](https://github.com/xbladestealth/grok-v2a/issues/3) for details and updates.
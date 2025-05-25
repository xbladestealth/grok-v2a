# Release Notes

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
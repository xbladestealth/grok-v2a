// Wrap all code in DOMContentLoaded to ensure DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    // Define default image path
    const DEFAULT_IMAGE_PATH = '/static/images/deca_bg.jpg';

    // Apply saved theme
    const themeSwitch = document.getElementById('theme-switch');
    const savedTheme = localStorage.getItem('theme');
    if (savedTheme === 'dark') {
        document.body.setAttribute('data-theme', 'dark');
        themeSwitch.checked = true;
        document.querySelector('link[rel="stylesheet"][href*="highlight.js"]').href =
            'https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/dark.min.css';
    }

    // Toggle theme
    themeSwitch.addEventListener('change', () => {
        if (themeSwitch.checked) {
            document.body.setAttribute('data-theme', 'dark');
            localStorage.setItem('theme', 'dark');
            document.querySelector('link[rel="stylesheet"][href*="highlight.js"]').href =
                'https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/dark.min.css';
        } else {
            document.body.removeAttribute('data-theme');
            localStorage.setItem('theme', 'light');
            document.querySelector('link[rel="stylesheet"][href*="highlight.js"]').href =
                'https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css';
        }
    });

    // Webcam mode setup
    let stream = null;
    let capturedImage = null;
    let buttonState = 'choose'; // choose, capture, restart
    const webcamSwitch = document.getElementById('webcam-switch');
    const video = document.querySelector('.preview-video');
    const previewImage = document.querySelector('.preview-image');
    const fileInput = document.getElementById('image');
    const fileButton = document.querySelector('.custom-file-button');

    // Ensure video is ready before allowing capture
    let videoReady = false;
    video.addEventListener('loadedmetadata', () => {
        videoReady = true;
        console.log('Video metadata loaded, ready to capture');
    });

    // Reset UI function
    function resetUI() {
        if (stream) {
            stream.getTracks().forEach(track => track.stop());
            stream = null;
        }
        video.srcObject = null;
        video.hidden = true;
        previewImage.hidden = false;
        previewImage.src = DEFAULT_IMAGE_PATH;
        fileInput.disabled = false;
        fileButton.textContent = 'Choose Image';
        fileButton.classList.remove('webcam-active');
        fileInput.value = '';
        capturedImage = null;
        videoReady = false;
        buttonState = 'choose';
        fileButton.setAttribute('aria-label', 'Choose Image');
        console.log('UI reset to default state');
        // Verify image load
        previewImage.onerror = () => {
            console.error('Failed to load default image:', DEFAULT_IMAGE_PATH);
            alert('Failed to load default image');
        };
    }

    // Webcam mode toggle
    webcamSwitch.addEventListener('change', async () => {
        if (webcamSwitch.checked) {
            try {
                stream = await navigator.mediaDevices.getUserMedia({ video: true });
                video.srcObject = stream;
                video.hidden = false;
                previewImage.hidden = true;
                fileInput.disabled = true;
                fileButton.textContent = 'Capture';
                fileButton.classList.add('webcam-active');
                fileInput.value = '';
                capturedImage = null;
                buttonState = 'capture';
                fileButton.setAttribute('aria-label', 'Capture Webcam Image');
                console.log('Webcam mode enabled');
            } catch (error) {
                alert(`Error accessing webcam: ${error.message}`);
                console.error('Webcam error:', error);
                webcamSwitch.checked = false;
                resetUI();
            }
        } else {
            resetUI();
        }
    });

    // Capture webcam image
    function captureWebcamImage() {
        console.log('Attempting to capture image');
        if (!videoReady || video.videoWidth === 0 || video.videoHeight === 0) {
            alert('Webcam is not ready. Please wait for the video feed to load.');
            console.warn('Capture failed: video not ready');
            return;
        }
        try {
            const canvas = document.createElement('canvas');
            canvas.width = video.videoWidth;
            canvas.height = video.videoHeight;
            const context = canvas.getContext('2d');
            if (!context) {
                throw new Error('Failed to get canvas context');
            }
            context.drawImage(video, 0, 0, canvas.width, canvas.height);
            const dataUrl = canvas.toDataURL('image/jpeg', 0.8);
            canvas.toBlob(blob => {
                if (!blob) {
                    alert('Failed to capture image blob');
                    console.error('Capture failed: no blob created');
                    resetUI();
                    return;
                }
                capturedImage = {
                    file: new File([blob], 'webcam-capture.jpg', { type: 'image/jpeg' }),
                    dataUrl: dataUrl
                };
                previewImage.src = dataUrl;
                previewImage.hidden = false;
                video.hidden = true;
                if (stream) {
                    stream.getTracks().forEach(track => track.stop());
                    stream = null;
                }
                video.srcObject = null;
                videoReady = false;
                fileButton.textContent = 'Restart';
                fileButton.classList.add('webcam-active');
                fileInput.disabled = false;
                buttonState = 'restart';
                fileButton.setAttribute('aria-label', 'Restart Webcam Feed');
                console.log('Image captured successfully');
                // Verify image load
                previewImage.onerror = () => {
                    console.error('Failed to load captured image');
                    alert('Failed to load captured image');
                };
            }, 'image/jpeg', 0.8);
        } catch (error) {
            alert(`Error capturing image: ${error.message}`);
            console.error('Capture error:', error);
            resetUI();
        }
    }

    // Restart webcam feed
    function restartWebcamFeed() {
        console.log('Restarting webcam feed');
        navigator.mediaDevices.getUserMedia({ video: true })
            .then(newStream => {
                stream = newStream;
                video.srcObject = stream;
                video.hidden = false;
                previewImage.hidden = true;
                fileButton.textContent = 'Capture';
                fileButton.classList.add('webcam-active');
                fileInput.disabled = true;
                buttonState = 'capture';
                fileButton.setAttribute('aria-label', 'Capture Webcam Image');
                console.log('Webcam feed restarted');
            })
            .catch(error => {
                alert(`Error restarting webcam: ${error.message}`);
                console.error('Restart error:', error);
                webcamSwitch.checked = false;
                resetUI();
            });
    }

    // Button click handler
    fileButton.addEventListener('click', (event) => {
        event.preventDefault();
        console.log('Button clicked, state:', buttonState);
        if (buttonState === 'capture') {
            captureWebcamImage();
        } else if (buttonState === 'restart') {
            restartWebcamFeed();
        } else {
            fileInput.click();
        }
    });

    // Update file input display and validate
    fileInput.addEventListener('change', (e) => {
        const file = e.target.files[0];
        
        if (!file) {
            fileButton.textContent = 'Choose Image';
            previewImage.src = capturedImage ? capturedImage.dataUrl : DEFAULT_IMAGE_PATH;
            fileButton.classList.remove('webcam-active');
            buttonState = 'choose';
            fileButton.setAttribute('aria-label', 'Choose Image');
            return;
        }
        
        if (!file.type.match(/^image\/(jpeg|jpg|png)$/)) {
            alert('Only JPEG or PNG images are supported');
            e.target.value = '';
            fileButton.textContent = 'Choose Image';
            previewImage.src = capturedImage ? capturedImage.dataUrl : DEFAULT_IMAGE_PATH;
            fileButton.classList.remove('webcam-active');
            buttonState = 'choose';
            fileButton.setAttribute('aria-label', 'Choose Image');
            return;
        }
        
        if (file.size > 5 * 1024 * 1024) {
            alert('Image size must be 5MB or less');
            e.target.value = '';
            fileButton.textContent = 'Choose Image';
            previewImage.src = capturedImage ? capturedImage.dataUrl : DEFAULT_IMAGE_PATH;
            fileButton.classList.remove('webcam-active');
            buttonState = 'choose';
            fileButton.setAttribute('aria-label', 'Choose Image');
            return;
        }
        
        const reader = new FileReader();
        reader.onload = function(e) {
            previewImage.src = e.target.result;
            capturedImage = null;
            fileButton.textContent = file.name;
            fileButton.classList.remove('webcam-active');
            buttonState = 'choose';
            fileButton.setAttribute('aria-label', 'Choose Image');
            console.log('File selected:', file.name);
            // Verify image load
            previewImage.onerror = () => {
                console.error('Failed to load uploaded image');
                alert('Failed to load uploaded image');
                previewImage.src = DEFAULT_IMAGE_PATH;
            };
        };
        reader.onerror = function() {
            alert('Error reading file');
            e.target.value = '';
            fileButton.textContent = 'Choose Image';
            previewImage.src = capturedImage ? capturedImage.dataUrl : DEFAULT_IMAGE_PATH;
            fileButton.classList.remove('webcam-active');
            buttonState = 'choose';
            fileButton.setAttribute('aria-label', 'Choose Image');
        };
        reader.readAsDataURL(file);
    });

    // Image Description Form Submission
    try {
        const uploadForm = document.getElementById('upload-form');
        if (!uploadForm) {
            console.error('Upload form not found');
            return;
        }
        uploadForm.addEventListener('submit', async (e) => {
            e.preventDefault();
            console.log('Image description form submitted');
            const form = e.target;
            const formData = new FormData(form);
            const responseContainer = document.getElementById('response-container');

            // Block submission when in capture state
            if (buttonState === 'capture') {
                alert('Please capture an image before submitting');
                console.log('Submission blocked: buttonState is capture');
                responseContainer.innerHTML = '';
                return;
            }

            responseContainer.innerHTML = '<p class="loading">Loading...</p>';

            // Use captured webcam image if available
            if (capturedImage && !formData.get('image').name) {
                formData.set('image', capturedImage.file, 'webcam-capture.jpg');
            }
            // Use default image if no file or webcam image
            else if (!formData.get('image').name) {
                try {
                    const defaultImageResponse = await fetch(DEFAULT_IMAGE_PATH);
                    if (!defaultImageResponse.ok) {
                        throw new Error('Failed to load default image');
                    }
                    const defaultImageBlob = await defaultImageResponse.blob();
                    formData.set('image', defaultImageBlob, 'deca_bg.jpg');
                } catch (error) {
                    alert(`Error loading default image: ${error.message}`);
                    console.error('Default image error:', error);
                    responseContainer.innerHTML = '';
                    return;
                }
            }

            try {
                const response = await fetch('/describe_image', {
                    method: 'POST',
                    body: formData,
                });
                const data = await response.json();
                if (response.ok) {
                    const markdownHtml = DOMPurify.sanitize(marked.parse(data.description));
                    responseContainer.innerHTML = `
                        <div id="response-text">${markdownHtml}</div>
                    `;
                    console.log('Image description loaded successfully');
                } else {
                    alert(`Error: ${data.error}`);
                    console.error('Submission error:', data.error);
                    responseContainer.innerHTML = '';
                }
            } catch (error) {
                alert(`Error: ${error.message}`);
                console.error('Submission error:', error);
                responseContainer.innerHTML = '';
            }
        });
    } catch (error) {
        console.error('Error setting up image description form listener:', error);
    }

    // Python Script Generator Form Submission
    try {
        const scriptForm = document.getElementById('script-form');
        if (!scriptForm) {
            console.error('Script form not found');
            return;
        }
        scriptForm.addEventListener('submit', async (e) => {
            e.preventDefault();
            console.log('Script generator form submitted');
            const form = e.target;
            const prompt = form.querySelector('#script-prompt').value;
            const responseContainer = document.getElementById('script-response-container');

            if (!prompt.trim()) {
                alert('Please enter a prompt for the Python script');
                console.log('Submission blocked: empty prompt');
                responseContainer.innerHTML = '';
                return;
            }

            responseContainer.innerHTML = '<p class="loading">Loading...</p>';

            try {
                const response = await fetch('/generate_python_script', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ prompt: prompt }),
                });
                const data = await response.json();
                if (response.ok) {
                    // Strip Markdown code block delimiters
                    let script = data.script;
                    script = script.replace(/^```python\s*\n/, '').replace(/\n```$/, '');
                    // Escape HTML to prevent XSS
                    const escapedScript = script
                        .replace(/</g, '&lt;')
                        .replace(/>/g, '&gt;');
                    responseContainer.innerHTML = `
                        <pre><code class="language-python">${escapedScript}</code></pre>
                    `;
                    hljs.highlightElement(responseContainer.querySelector('code'));
                    console.log('Python script loaded successfully');
                } else {
                    alert(`Error: ${data.error}`);
                    console.error('Submission error:', data.error);
                    responseContainer.innerHTML = '';
                }
            } catch (error) {
                alert(`Error: ${error.message}`);
                console.error('Submission error:', error);
                responseContainer.innerHTML = '';
            }
        });
    } catch (error) {
        console.error('Error setting up script form listener:', error);
    }
});
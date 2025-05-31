// Define default image path
const DEFAULT_IMAGE_PATH = '/static/images/deca_bg.jpg';

// Apply saved theme
const themeSwitch = document.getElementById('theme-switch');
const savedTheme = localStorage.getItem('theme');
if (savedTheme === 'dark') {
    document.body.setAttribute('data-theme', 'dark');
    themeSwitch.checked = true;
}

// Toggle theme
themeSwitch.addEventListener('change', () => {
    if (themeSwitch.checked) {
        document.body.setAttribute('data-theme', 'dark');
        localStorage.setItem('theme', 'dark');
    } else {
        document.body.removeAttribute('data-theme');
        localStorage.setItem('theme', 'light');
    }
});

// Update file input display and validate
document.getElementById('image').addEventListener('change', (e) => {
    const file = e.target.files[0];
    const button = document.querySelector('.custom-file-button');
    
    if (!file) {
        button.textContent = 'No file chosen';
        const img = document.querySelector('.preview-image');
        img.src = DEFAULT_IMAGE_PATH; // Reset to default image
        return;
    }
    
    if (!file.type.match(/^image\/(jpeg|jpg|png)$/)) {
        alert('Only JPEG or PNG images are supported');
        e.target.value = '';
        button.textContent = 'Choose Image';
        const img = document.querySelector('.preview-image');
        img.src = DEFAULT_IMAGE_PATH; // Reset to default image
        return;
    }
    
    if (file.size > 5 * 1024 * 1024) {
        alert('Image size must be 5MB or less');
        e.target.value = '';
        button.textContent = 'Choose Image';
        const img = document.querySelector('.preview-image');
        img.src = DEFAULT_IMAGE_PATH; // Reset to default image
        return;
    }
    
    const reader = new FileReader();
    reader.onload = function(e) {
        const img = document.querySelector('.preview-image');
        img.src = e.target.result;
    };
    reader.onerror = function() {
        alert('Error reading file');
        e.target.value = '';
        button.textContent = 'Choose Image';
        const img = document.querySelector('.preview-image');
        img.src = DEFAULT_IMAGE_PATH; // Reset to default image
    };
    reader.readAsDataURL(file);
    button.textContent = file.name;
});

// Form submission
document.getElementById('upload-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const form = e.target;
    const formData = new FormData(form);
    const responseContainer = document.getElementById('response-container');
    responseContainer.innerHTML = '<p class="loading">Loading...</p>';

    // Check if an image file is selected; if not, append default image
    if (!formData.get('image').name) {
        try {
            const defaultImageResponse = await fetch(DEFAULT_IMAGE_PATH);
            if (!defaultImageResponse.ok) {
                throw new Error('Failed to load default image');
            }
            const defaultImageBlob = await defaultImageResponse.blob();
            formData.set('image', defaultImageBlob, 'deca_bg.jpg'); // Use set to replace any empty file
        } catch (error) {
            alert(`Error loading default image: ${error.message}`);
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
        } else {
            alert(`Error: ${data.error}`);
            responseContainer.innerHTML = '';
        }
    } catch (error) {
        alert(`Error: ${error.message}`);
        responseContainer.innerHTML = '';
    }
});
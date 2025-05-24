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
        return;
    }
    if (!file.type.startsWith('image/jpeg') && !file.type.startsWith('image/png')) {
        alert('Only JPEG or PNG images are supported');
        e.target.value = '';
        button.textContent = 'Choose Image';
        return;
    }
    if (file.size > 5 * 1024 * 1024) {
        alert('Image size must be 5MB or less');
        e.target.value = '';
        button.textContent = 'Choose Image';
        return;
    }
    button.textContent = file.name;
});

// Form submission
document.getElementById('upload-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const form = e.target;
    const formData = new FormData(form);
    const responseContainer = document.getElementById('response-container');
    responseContainer.innerHTML = '<p class="loading">Loading...</p>';

    try {
        const response = await fetch('/describe_image', {
            method: 'POST',
            body: formData,
        });
        const data = await response.json();
        if (response.ok) {
            const markdownHtml = DOMPurify.sanitize(marked.parse(data.description));
            responseContainer.innerHTML = `
                <img id="response-image" src="${data.image_url}" alt="Uploaded image">
                <div id="response-text">${markdownHtml}</div>
            `;
        } else {
            responseContainer.innerHTML = `<p class="error">Error: ${data.error}</p>`;
        }
    } catch (error) {
        responseContainer.innerHTML = `<p class="error">Error: ${error.message}</p>`;
    }
});
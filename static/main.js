const fieldsInput = document.getElementById('fields');

document.forms.mainform.addEventListener('submit', () => {
    fieldsInput.value = fieldsInput.value
        .split(',')
        .map(el => el.trim())
        .join(',')
        .slice(0, -1); // Remove last char
});

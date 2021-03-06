const fieldsInput = document.getElementById('fields');
const closeBtn = document.getElementById('close-button');

document.forms.mainform.addEventListener('submit', () => {
  fieldsInput.value = fieldsInput.value
    .split(',')
    .map(el => el.trim())
    .join(',')
    .slice(0, -1); // Remove last char
});

closeBtn.addEventListener('click', () => fetch('/close'));

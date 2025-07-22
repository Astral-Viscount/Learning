const button = document.getElementById('button');
const text = document.getElementById('text');

button.addEventListener('click', function() {
    text.textContent = 'Hello, World!';
});
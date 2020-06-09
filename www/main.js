import wasm from '../parse-mail/Cargo.toml';

/**
 * Split array into constant size chunks
 * @param {Array<any>} arr
 * @param {Number} chunkSize
 * @returns {Array<Array<any>>}
 */
const chunks = (arr, chunkSize) =>
  new Array(Math.ceil(arr.length / chunkSize))
    .fill()
    .map((_, i) => arr.slice(i * chunkSize, i * chunkSize + chunkSize));

/**
 * @param {string} str
 * @returns {Object}
 */
const parse = str => Object.fromEntries(chunks(wasm.parse_mail(str), 2));

function addField(name) {
  document.getElementById('fields').innerHTML += `, ${name}`;
}

const template = document.getElementById('field-template');

document.getElementById('example-file-input').addEventListener('change', async ev => {
  const [file] = ev.target.files;
  const textContent = await file.text();
  const fields = Object.keys(parse(textContent));

  document.querySelectorAll('.example-doc__fields').forEach(el => {
    fields.forEach(field => {
      const currentElement = template.cloneNode(true);
      currentElement.querySelector('.field-template__field-name').innerHTML = field;

      el.innerHTML += currentElement.innerHTML;
    });

    el.querySelectorAll('.field-template__action').forEach(el => {
      el.addEventListener('click', ({ target: { parentElement } }) => {
        const { innerHTML: fieldName } = parentElement.querySelector('.field-template__field-name');
        addField(fieldName);
      });
    });
  });
});

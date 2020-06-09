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

const parse = str => Object.fromEntries(chunks(wasm.parse_mail(str), 2));

const txt = `
imie:
asdd

nazwisko:
ads

dsa
dsa
dasd
as


das
d

`;

console.log(parse(txt));

// const fieldsInput = document.getElementById('fields');
// const closeBtn = document.getElementById('close-button');

// document.forms.mainform.addEventListener('submit', c => {
//   fieldsInput.value = fieldsInput.value
//     .split(',')
//     .map(el => el.trim())
//     .join(',')
//     .slice(0, -1); // Remove last char
// });

// closeBtn.addEventListener('click', () => fetch('/close'));

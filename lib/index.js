const addon = require('../native');
const filepath = process.argv[2] || './data/shakespeare-plays.csv';

console.log('File:', filepath);
console.log('Will issue word counting...');

console.time('Word counting time');
addon.count_words(filepath, (err, result) => {
    console.timeEnd('Word counting time');
    if (err) console.log('Failed:', err);
    else console.log('Result:', result);
});

console.log('Awaiting for results...');

module.exports = addon;
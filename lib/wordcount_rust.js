const vc = require('.');
const filepath = process.argv[2] || './data/shakespeare-plays.csv';

console.log('Implementation: Rust');
console.log('File:', filepath);
console.log('Will issue word counting...');

console.time('Word counting time');
vc.count_words(filepath, (err, result) => {
    if (err) console.log('Failed:', err);
    else console.log('Result:', result);
    console.timeEnd('Word counting time');
});

console.log('Awaiting for results...');

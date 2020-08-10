const fs = require('fs');
const { group } = require('console');
const filepath = process.argv[2] || './data/shakespeare-plays.csv';

console.log('Implementation: Rust');
console.log('File:', filepath);
console.log('Will issue word counting...');

console.time('Word counting time');
countWords(filepath, (err, result) => {
    if (err) console.log('Failed:', err);
    else console.log('Result:', result);
    console.timeEnd('Word counting time');
});

console.log('Awaiting for results...');

// Node implementation
//

function countWords(filepath, callback) {
    fs.readFile(filepath, 'utf-8', (err, content) => {
        const words = content.split(' ');
        const counted = words.reduce((map, word) => {
            map[word] = (map[word] || 0) + 1;
            return map;
        }, {});

        const sorted = [];
        for (let word in counted) {
            sorted.push({ word: word, count: counted[word] })
        }
        sorted.sort((a, b) => b.count - a.count);

        callback(null, { data: sorted });
    });
}
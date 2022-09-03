const addon = require('../native');

// console.log(addon.hello());

module.exports = {
    hello: addon.hello,
    count_words: addon.count_words
}

const MESSAGE = 'Hello, World!';

function greet() {
    console.log(MESSAGE);
}

/**
 * 
 * @ztpExport
 * @param {string} name 
 * @param {number} count 
 * @returns {Promise<string>}
 */
function doSomethingRemotely(name, count) {
    'use ztp';
    return new Promise((resolve) => {
        setTimeout(() => {
            resolve(`Hello, ${name}! You have ${count} new messages.`);
        }, 1000);
    });
}

greet();

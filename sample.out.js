var MESSAGE = "Hello, World!";
function greet() {
    console.log(MESSAGE);
}
/**
 * 
 * @ztpExport
 * @param {string} name 
 * @param {number} count 
 * @returns {Promise<string>}
 */ function doSomethingRemotely(name: any, count: any) {
    return new Promise(function(resolve: any) {
        setTimeout(function() {
            resolve("Hello, ".concat(name, "! You have ").concat(count, " new messages."));
        }, 1000);
    });
}
greet();

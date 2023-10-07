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
 */ function doSomethingRemotely(name, count) {
    __TO_ZTP_SANDBOX({
        name: "doSomethingRemotely",
        params: [
            name,
            count
        ]
    });
}
greet();

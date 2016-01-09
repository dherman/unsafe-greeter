var addon = require('../native');

function Greeter(greeting) {
  const greeter = addon.create_greeter(greeting);

  function hello(name) {
    return addon.greeter_hello(greeter, name);
  }

  return Object.freeze({
    hello
  });
}

module.exports = Greeter;

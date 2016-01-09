#[macro_use]
extern crate neon;

use std::mem::transmute;

use neon::vm;
use neon::vm::{Call, JsResult};
use neon::mem::Handle;
use neon::js::{JsUndefined, JsString};
use neon::js::binary::JsBuffer;

struct Greeter {
    greeting: String
}

impl Greeter {
    fn new(greeting: String) -> Greeter {
        Greeter {
            greeting: greeting
        }
    }

    fn hello(&self, name: &str) -> String {
        format!("{}, {}!", self.greeting, name)
    }

    // DANGER: This is terrible and you should not ever do this!
    //         Dave is working on an actual safe API.
    unsafe fn from_buffer(buffer: Handle<JsBuffer>) -> Box<Greeter> {
        let p: *mut Greeter = read_pointer(buffer);
        Box::from_raw(transmute(p))
    }
}

// DANGER: This is terrible and you should not ever do this!
//         Dave is working on an actual safe API.
unsafe fn write_pointer<T>(p: *mut T, buffer: Handle<JsBuffer>) {
    let i = p as u64;
    vm::lock(buffer, |mut data| {
        let bytesp: &mut [u8] = data.as_mut_slice().unwrap();
        let wordsp: &mut [u64] = transmute(bytesp);
        wordsp[0] = i;
    });
}

// DANGER: This is terrible and you should not ever do this!
//         Dave is working on an actual safe API.
unsafe fn read_pointer<T>(buffer: Handle<JsBuffer>) -> *mut T {
    let i = vm::lock(buffer, |data| {
        let bytesp: &[u8] = data.as_slice().unwrap();
        let wordsp: &[u64] = transmute(bytesp);
        wordsp[0]
    });
    i as *mut T
}

// DANGER: This is terrible and you should not ever do this!
//         Dave is working on an actual safe API.
fn create_greeter(call: Call) -> JsResult<JsBuffer> {
    let buffer = try!(JsBuffer::new(call.scope, 8));
    let greeting = try!(try!(call.arguments.require(call.scope, 0)).check::<JsString>()).data();
    let greeter = Box::new(Greeter::new(greeting));
    unsafe {
        write_pointer(Box::into_raw(greeter), buffer);
    }
    Ok(buffer)
}

// DANGER: This is terrible and you should not ever do this!
//         Dave is working on an actual safe API.
fn greeter_hello(call: Call) -> JsResult<JsString> {
    let buffer = try!(try!(call.arguments.require(call.scope, 0)).check::<JsBuffer>());
    let name = try!(try!(call.arguments.require(call.scope, 1)).check::<JsString>()).data();
    let greeter = unsafe { Greeter::from_buffer(buffer) };
    let hello = greeter.hello(&name[..]);
    JsString::new_or_throw(call.scope, &hello[..])
}

// DANGER: This is terrible and you should not ever do this!
//         Dave is working on an actual safe API.
fn drop_greeter(call: Call) -> JsResult<JsUndefined> {
    let buffer = try!(try!(call.arguments.require(call.scope, 0)).check::<JsBuffer>());
    unsafe {
        Greeter::from_buffer(buffer);
    }
    Ok(JsUndefined::new())
}

register_module!(m, {
    try!(m.export("create_greeter", create_greeter));
    try!(m.export("greeter_hello", greeter_hello));
    try!(m.export("drop_greeter", drop_greeter));
    Ok(())
});

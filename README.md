# rust-wasm-test
test repo to figure out how to make a full stack web app with Rust, WASM and other technologies

## what do we need
- some server to serve the binaries, html as scaffolding
- some type of javascript interop

## what we used
- actix web server + maud html template
- wasm-bindgen for client functionality
- wasm-pack -> pkg -> serve

Overall, had to jump through a lot of hoops to set this up. Doesn't feel that good having to import all of your wasm functions through the preescape. Feels a bit wonky to define html stuff in one place and the functionality in another, but I guess that's how web dev is these days. 

What I want to know, is if there is a more streamlined method of doing things. I want to throw up a canvas, have some input functionality and then have the Rust code do the rest, without this constant back and forth between the js and wasm layers for basic things.





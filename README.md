# Learning Sycamore in 100 Days

This is my Sycamore learning diary. [Sycamore](https://sycamore-rs.netlify.app) is a Rust framework for writing web sites and web apps in [WebAssembly](https://webassembly.org/).

## App Structure

I've created a frontend (which is the Sycamore site) and a server. The server is powered by [Rocket](https://rocket.rs/) which is a web framework for writing web applications in Rust. The two components of this Rust crate are unified via the notion of Cargo workspaces.

## Building

To compile the front facing site, follow the instructions on the [Getting Started](https://sycamore-rs.netlify.app/docs/v0.8/getting_started/installation) page available on the Sycamore site. This will get you set up with the tools needed to build for WebAssembly.

Next, from a command line located in the crate root, type:

```
cd frontend
trunk build --release
```

This will use the trunk tool to build the site. Output is placed in the /dist folder.

## Running

After a fresh copy of the site is built, type:

```
cd ../server
cargo run
```

The rocket server will start and you'll be presented with a variaty of status messages. At the end, you'll see the site available at: http://127.0.0.1:8000. Go to that link in your browser and the Scyamore built web page should be available.

# toolbelt
## Setup
First we need to install all the necessary tools

+ [Install Rust and wasm-pack](https://rustwasm.github.io/wasm-pack/)
+ [Install Nodejs and npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

Clone the repository and download the node modules for svelte

```console
foo@bar:~$ git clone https://github.com/helvete003/toolbelt.git
foo@bar:~$ cd toolbelt/www
foo@bar:~/toolbelt/www$ npm install
```

## Compiling
#### Warning
Currently you have to manually copy the bootstrap icons fonts folder `(node_modules/bootstrap-icons/font/fonts)` into the build folder `(public/build)`
### Debug
Build the WASM module
```console
foo@bar:~/toolbelt$ wasm-pack build --target no-modules --out-dir www/public/pkg --no-typescript
```
Build Svelte and start dev server
```console
foo@bar:~/toolbelt/www$ npm run dev
```
### Release
```console
foo@bar:~/toolbelt$ wasm-pack build --target no-modules --out-dir www/public/pkg --no-typescript --release
```
Build the Svelte Project
```console
foo@bar:~/toolbelt/www$ npm run build
```
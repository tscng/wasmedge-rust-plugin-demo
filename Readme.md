WasmEdge Rust Plugin Demonstration
===
This repository validates a minimal rust -> wasm deployment workflow with plugin integration.  
The rust program uses WasmEdge's `process` plugin to output the current dir contents using `ls`.

## CI/CD
This repository contains sample GitHub workflows for building wasm binaries as well as a wasm container image.  
The image is created from scratch, only containing the wasm binary, and build for the `wasi/wasm` [platform](https://docs.docker.com/build/ci/github-actions/multi-platform/).  
Furthermore, for compatibility, it is also [annotated](https://docs.docker.com/build/ci/github-actions/annotations/) with `"module.wasm.image/variant": "compat-smart"`.

A separate workflow builds the wasm binary from rust directly in the GitHub runner environment.  
The binary is added as release asset.

## Plugin
To make use of the plugin, it needs to be installed in the WasmEdge [plugin directory](https://wasmedge.org/docs/contribute/plugin/intro/#loading-plug-in-from-paths). 
At the time of writing, this command can be used to download the plugin from the build assets for Ubuntu without having to build WasmEdge locally:  
```
wget -qO- https://github.com/WasmEdge/WasmEdge/releases/latest/download/WasmEdge-plugin-wasmedge_process-0.15.0-ubuntu20.04_x86_64.tar.gz | sudo gunzip | sudo tar xvf -
```

Verify the plugin using `wasmedge -v`.

## Run Wasm binary via WasmEdge
To run the plugin, download the compiled wasm binary:  
```
wget https://github.com/tscng/wasmedge-rust-plugin-demo/releases/latest/download/wasmedge-rust-plugin-demo.wasm
```

The `process` plugin needs whitelisting of commands.  
Execute it using WasmEdge:
```
wasmedge run --allow-command=ls wasmedge-rust-plugin-demo.wasm
```

## Run Wasm container via Podman
Podman is my personally preferred way to run wasm containers while avoiding a docker desktop installation.  
The bare docker engine does not support wasi/wasm architecture at the time of writing.  
Make sure to [set up Podman](https://wasmedge.org/docs/develop/deploy/podman/): 
- Install Podman via apt-get
- Install WasmEdge via their installer script
- Install crun with WasmEdge support by building it locally from source

Also, make sure to include the `process` plugin as described above.
Run the container:
```
sudo podman run --rm --platform=wasi/wasm ghcr.io/tscng/wasmedge-rust-plugin-demo:latest
```

This will likely fail with the error that the `ls` command is not whitelisted, but still validates the working plugin setup.
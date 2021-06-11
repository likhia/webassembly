# Simple WASM extension written in Rust

## Steps to build it

Install `rust`, if not already installed:

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install `wasm32` target, if not already installed:

```sh
$ rustup target add wasm32-unknown-unknown
```

Build the extension or to compile your code:

```sh
$ make build
```

A file named `extension.wasm` was created in the current directory.


Open a Terminal and login to your openshift cluster.  
``` 
REGISTRY=<Internal Registry in Openshift>

docker login -u kubeadmin -p $(oc whoami -t) $REGISTRY
```

```sh
$ make container
```

This will create the container image into local docker 

```
docker tag proxy-filter:latest $REGISTRY/istio-system/proxy-filter

docker push $REGISTRY/istio-system/proxy-filter
```
This will push the container image to the internal registry of your openshift cluster. 


```
vi ../servicemesh/proxyFilterExtension.yaml
```
Replace the SHA generated in the earlier step.  

Please remember to enable wasmExtension in OSM control plane if you have not done it. Please wait till all the conditions are true then proceed to next step. 

techPreview:
    wasmExtensions:
      enabled: true

```
oc apply -f ../servicemesh/proxyFilterExtension.yaml 
```
This will create / update the servicemeshextension.



# Solace Semp Client

Build the Solace Sempv2 API Client from OpenAPI Spec

## Building

### Updating SEMP API OpenAPI Spec

Download the appropriate version of the appliance SEMPv2 OpenAPI Spec from sftp.solacesystems.com, and place it in 

* config/__VERSION__/semp-v2-swagger-config.yaml
* config/__VERSION__/semp-v2-swagger-action.yaml
* config/__VERSION__/semp-v2-swagger-monitor.yaml


### Building

Run the build script, passing in the language, SEMP version, and target version. 

Note for rust, you need to specify a suitable target version like 9.0.1-30 due to how semver is implemented. Most other 
languages seem fine with 3 separator versions. 

```bash
./build.sh [java|python|rust|swift] src_version target_version
```

#### Rust

Due to how rust implements semver, you want to specify the target version using only 2 dot separators. e.g 9.0.1.30 becomes 9.0.1-30

    ./build.sh rust 9.5.0.30 9.5.0-30  

### Python Via Docker Run

Build the python wheel.

    ./build.sh python 9.8.0.12
    
    # example 1
    ls output/9.8.0.12 | xargs -I@  docker run -t -v `pwd`:/src python:2.7-slim /src/venv-wrapper.sh "cd /src/output/9.8.0.12/@ && python setup.py bdist_wheel --universal"  
    
    # example py2
    docker run -t -v `pwd`:/src python:2.7-slim /src/venv-wrapper.sh "cd /src/output/VERSION/python_[config|action|monitor] && python setup.py bdist_wheel --universal"
    
    # example py3
    docker run -t -v `pwd`:/src python:3-slim /src/venv-wrapper.sh "cd /src/output/VERSION/python_[config|action|monitor] && python setup.py bdist_wheel --universal"
    

You can now find the Wheel files output/VERSION/python_[action|config|monitor]/dist/

#### Python via Docker Build

    touch .pypirc
    docker build --build-arg upload=0 --build-arg sempver=9.8.0.12 -t unixunion/disposable:dev-9.8.0.12 .
    # create a container and get its ID    
    docker create unixunion/disposable:dev-9.8.0.12
    # copy the wheel from the container
    docker cp CONTAINER_ID:/tmp output

#### Building and Releasing all Python Wheel artefacts

    ls config | xargs -I@ -t docker build --build-arg upload=1 --build-arg sempver=@ -t unixunion/disposable:dev-@ .

### Getting all SEMPv2 client whl files

After running above, you can extract ALL wheel files with: 

    ls config | xargs -I@ -t docker create unixunion/disposable:dev-@ | xargs -I@ docker cp @:/tmp output


## Generator Configs

If you need to adjust the config for codegen, you can find generator configurable
parameters with:

    docker run -ti swaggerapi/swagger-codegen-cli:2.4.2 config-help -l python
    docker run -ti swaggerapi/swagger-codegen-cli:2.4.2 config-help -l java
    docker run -ti swaggerapi/swagger-codegen-cli:2.4.2 config-help -l rust
    docker run -ti swaggerapi/swagger-codegen-cli:2.4.2 config-help -l swift


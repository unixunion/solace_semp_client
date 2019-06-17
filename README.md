# Solace Semp Client

Build the Solace Sempv2 API Client from OpenAPI Spec

## Building

### Updating Semp API

Download the appropriate version of the appliance SEMPv2 OpenAPI Spec from sftp.solacesystems.com, and place it in `config/__VERSION__/semp-v2-swagger-config.yaml`

## Manually

### Building

Run the build script, passing in the language, semp version, and target version. 

Note for rust, you need to specify a suitable target version like 9.0.1-30 due to how semver is implemented. Most other languages seem fine with 3 separator versions. 

```bash
./build.sh [java|python|rust|swift] src_version target_version
```

#### Rust

Due to how rust implementes semver, you want to specify the target version using only 2 dot separators. e.g 9.0.1.30 becomes 9.0.1-30

### Building Python Wheel

Build the python wheel.

    ./build.sh python 9.0.1.30 9.0.1.30
    # py2
    docker run -t -v `pwd`:/src python:2.7-slim /src/venv-wrapper.sh "cd /src/output/python && python setup.py bdist_wheel --universal"
    # py3
    docker run -t -v `pwd`:/src python:3-slim /src/venv-wrapper.sh "cd /src/output/python && python setup.py bdist_wheel --universal"

You can now find the Wheel file in output/python/dist/


## Generator Config

If you need to adjust the config for codegen, you can find generator configurable
parameters with:

    docker run -ti swaggerapi/swagger-codegen-cli:2.4.2 config-help -l python
    docker run -ti swaggerapi/swagger-codegen-cli:2.4.2 config-help -l java
    docker run -ti swaggerapi/swagger-codegen-cli:2.4.2 config-help -l rust
    docker run -ti swaggerapi/swagger-codegen-cli:2.4.2 config-help -l swift

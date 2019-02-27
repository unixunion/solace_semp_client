# Solace Semp Client

Build the Solace Sempv2 API Client from OpenAPI Specifications.

## Building

### Updating Semp API

Download the appropriate version of the appliance sempv2 OpenAPI spec from sftp.solacesystems.com, place
it in `config/__VERSION__/semp-v2-swagger-config.yaml`

## Manually

### Configuring Codegen

Create a suitable java and python config for Codegen using templates, setting
the version and naming them `config-java.json` and `config-python.json` respectively.

### Python

Build the python wheel.

    cat config-python.json.template | sed 's/__VERSION__/9.0.1.7/' > config-python.json
    docker run -v `pwd`:/src swaggerapi/swagger-codegen-cli:2.4.2 generate \
      --config /src/config-python.json \
      -l python \
      -i /src/config/9.0.1.7/semp-v2-swagger-config.yaml \
      -o /src/output/python
    # py2
    docker run -t -v `pwd`:/src python:2.7-slim /src/venv-wrapper.sh "cd /src/output/python && python setup.py bdist_wheel --universal"
    # py3
    docker run -t -v `pwd`:/src python:3-slim /src/venv-wrapper.sh "cd /src/output/python && python setup.py bdist_wheel --universal"

You can now find the Wheel file in output/python/dist/


### Building

```bash
./build.sh [java|python|rust|swift]
```



## Generating Documentation



## Generator Config

If you need to adjust the config for codegen, you can find generator configurable
parameters with:

    docker run -ti swaggerapi/swagger-codegen-cli:2.4.2 config-help -l python
    docker run -ti swaggerapi/swagger-codegen-cli:2.4.2 config-help -l java
    docker run -ti swaggerapi/swagger-codegen-cli:2.4.2 config-help -l rust
    docker run -ti swaggerapi/swagger-codegen-cli:2.4.2 config-help -l swift

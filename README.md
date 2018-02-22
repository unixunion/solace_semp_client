# Solace Semp Client

This tool builds API Clients from OpenAPI specifications using the Codegen container.

## Building

### Configuring Codegen

Download the latest Appliance sempv2 OpenAPI spec from sftp.solacesystems.com, place
it in `config/semp-v2-swagger-config.yaml`

Create a suitable java and python config for Codegen using templates, setting
the version and naming them `config-java.json` and `config-python.json` respectively.

### Python

    cat config-python.json.template | sed 's/__VERSION__/0.0.1/' > config-python.json
    docker run -v `pwd`:/src swaggerapi/swagger-codegen-cli:v2.3.1 generate \
      --config /src/config-python.json \
      -l python \
      -i /src/config/semp-v2-swagger-config.yaml \
      -o /src/output/python
    docker run -t -v `pwd`:/src python:2.7-slim /src/venv-wrapper.sh "cd /src/output/python && python setup.py bdist_wheel --universal"


### Java

    cat config-java.json.template | sed 's/__VERSION__/0.0.1/' > config-java.json
    docker run -v `pwd`:/src swaggerapi/swagger-codegen-cli:v2.3.1 generate \
      --config /src/config-java.json \
      -l java \
      -i /src/config/semp-v2-swagger-config.yaml \
      -o /src/output/java
    cd output/java
    mvn clean / package / install / deploy

## Generator Config

You can find generator config parameters with:

    docker run -ti swaggerapi/swagger-codegen-cli:v2.3.1 config-help -l python
    docker run -ti swaggerapi/swagger-codegen-cli:v2.3.1 config-help -l java

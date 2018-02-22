# Solace Semp Client

This tool builds API Clients from OpenAPI specifications using the Codegen container.



## Building

### Updating Semp API

Download the latest Appliance sempv2 OpenAPI spec from sftp.solacesystems.com, place
it in `config/semp-v2-swagger-config.yaml` commit and push.

There is a pipeline

### Pipeline

The pipeline should automatically pick up the change, run codegen, compile the
python code and release it to artifactory.

https://go-cd.arch.aws.unibet.com/go/tab/pipeline/history/SolaceSempClient-OpenAPIGenerate


## Manually

### Configuring Codegen

Create a suitable java and python config for Codegen using templates, setting
the version and naming them `config-java.json` and `config-python.json` respectively.

### Python

Build the python wheel.

    cat config-python.json.template | sed 's/__VERSION__/0.0.1/' > config-python.json
    docker run -v `pwd`:/src swaggerapi/swagger-codegen-cli:v2.3.1 generate \
      --config /src/config-python.json \
      -l python \
      -i /src/config/semp-v2-swagger-config.yaml \
      -o /src/output/python
    docker run -t -v `pwd`:/src python:2.7-slim /src/venv-wrapper.sh "cd /src/output/python && python setup.py bdist_wheel --universal"

You can now find the Wheel file in output/python/dist/

### Java

If required, you can build a java version too.

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

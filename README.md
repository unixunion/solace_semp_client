# Solace Semp Client

This tool builds API Clients from OpenAPI specifications.

## Building

### Config

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
    cd output/python
    virtualenv venv
    source venv/bin/activate
    pip install -r requirements.txt
    python setup.py ...

### Java

    cat config-java.json.template | sed 's/__VERSION__/0.0.1/' > config-java.json
    docker run -v `pwd`:/src swaggerapi/swagger-codegen-cli:v2.3.1 generate \
      --config /src/config-java.json \
      -l java \
      -i /src/config/semp-v2-swagger-config.yaml \
      -o /src/output/java
    cd output/java
    mvn clean package install

## Generator Config

You can find generator config parameters with:

    docker run -ti swaggerapi/swagger-codegen-cli:v2.3.1 config-help -l python
    docker run -ti swaggerapi/swagger-codegen-cli:v2.3.1 config-help -l java

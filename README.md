# Solace Semp Client

This tool builds API Clients from OpenAPI specifications.

## Building

Get the latest semp-v2-swagger-config.yaml from sftp.solacesystems.com, place in config/

    paver build_python_client
    paver build_java_client

## Generator Config

You can find generator config parameters with:

    docker run -ti swaggerapi/swagger-codegen-cli:v2.3.1 config-help -l python
    docker run -ti swaggerapi/swagger-codegen-cli:v2.3.1 config-help -l java

# simple wrapper to load the yaml into the container
FROM swaggerapi/swagger-codegen-cli:v2.3.1
COPY config/semp-v2-swagger-config.yaml /editor/spec-files/
COPY config.json /editor/spec-files/

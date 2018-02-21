# simple wrapper to load the yaml into the container
FROM swaggerapi/swagger-codegen-cli:v2.3.1
COPY default.yaml /editor/spec-files/
COPY config.json /editor/spec-files/

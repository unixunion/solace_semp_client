#!/bin/bash

target=$1;

cat config-${target}.json.template | sed 's/__VERSION__/9.0.1.7/' > config-${target}.json
docker run -v `pwd`:/src swaggerapi/swagger-codegen-cli:2.4.2 generate \
    --config /src/config-${target}.json \
    -l ${target} \
    -i /src/config/9.0.1.7/semp-v2-swagger-config.yaml \
    -o /src/output/${target}
cd output/${target}
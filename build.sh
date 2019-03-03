#!/bin/bash

target=$1;
version=$2;
rewrite_version=$3;

usage() {
  echo "usage"
  echo "build.sh lang semp_version [rewrite_version]"
  echo "./build.sh rust 9.0.1.7 9.0.1-7"
}

if [ "$target" == "" ]; then
  echo "no target specified"
  usage
  exit 1
fi

if [ "$version" == "" ]; then
  echo "no version specified"
  usage
  exit 1
fi

if [ "$rewrite_version" == "" ]; then 
  rewrite_version=${version}
fi

if [ ! -f "config/$version/semp-v2-swagger-config.yaml" ]; then
  echo "no swagger spec found: config/$version/semp-v2-swagger-config.yaml"
  usage
  exit 1
fi

cat config-${target}.json.template | sed 's/__VERSION__/${rewrite_version}/' > config-${target}.json
docker run -v `pwd`:/src swaggerapi/swagger-codegen-cli:2.4.2 generate \
    --config /src/config-${target}.json \
    -l ${target} \
    -i /src/config/${version}/semp-v2-swagger-config.yaml \
    -o /src/output/${target}
cd output/${target}
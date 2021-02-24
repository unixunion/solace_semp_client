#!/bin/bash

target=$1;
version=$2;
rewrite_version=$3;

usage() {
  echo "usage"
  echo "build.sh lang semp_version [rewrite_version]"
  echo "./build.sh rust 9.1.0.77 9.1.0-77"
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

exargs=""

if [ "${target}" == "rust" ]; then
  exargs="-t /src/swagger_templates/${target}"
fi

cat config-${target}.json.template | sed "s/__VERSION__/${rewrite_version}/" > config-${target}.json
docker run -v `pwd`:/src swaggerapi/swagger-codegen-cli:2.4.13 generate \
    --config /src/config-${target}.json \
    -l ${target} \
    -i /src/config/${version}/semp-v2-swagger-config.yaml \
    ${exargs} \
    -o /src/output/${version}/${target}_config

if [ -f "config/$version/semp-v2-swagger-monitor.yaml" ]; then

  cat monitor-${target}.json.template | sed "s/__VERSION__/${rewrite_version}/" > monitor-${target}.json
  docker run -v `pwd`:/src swaggerapi/swagger-codegen-cli:2.4.13 generate \
      --config /src/monitor-${target}.json \
      -l ${target} \
      -i /src/config/${version}/semp-v2-swagger-monitor.yaml \
      ${exargs} \
      -o /src/output/${version}/${target}_monitor

fi

if [ -f "config/$version/semp-v2-swagger-action.yaml" ]; then
  cat action-${target}.json.template | sed "s/__VERSION__/${rewrite_version}/" > action-${target}.json
  docker run -v `pwd`:/src swaggerapi/swagger-codegen-cli:2.4.13 generate \
      --config /src/action-${target}.json \
      -l ${target} \
      -i /src/config/${version}/semp-v2-swagger-action.yaml \
      ${exargs} \
      -o /src/output/${version}/${target}_action

fi

if [ "$target" == "python" ]; then
  for module in output/${version}/*; do
    echo $module
    docker run -t -v `pwd`:/src python:3-slim /src/venv-wrapper.sh "cd /src/$module && python setup.py bdist_wheel --universal"
  done
fi
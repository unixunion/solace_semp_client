#!/usr/bin/env bash

# This runs inside a python-slim container ensure a virtualenv is setup
# and sources before building the codegen produced code.

apt-get update && apt-get -y install python-virtualenv

if [ -d "venv" ]; then
  echo "found python virtualenv"
else
  echo "creating python virtualenv"
  virtualenv venv
  source venv/bin/activate
  pip install -r /src/output/python/requirements.txt
  pip install wheel twine
fi

source venv/bin/activate

bash -c "$@"

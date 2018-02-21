#!/usr/bin/env bash

if [ -d "venv" ]; then
  echo "found python virtualenv"
else
  echo "creating python virtualenv"
  virtualenv venv --python=python3
  source venv/bin/activate
  pip install -r requirements.txt
fi

source venv/bin/activate

$@

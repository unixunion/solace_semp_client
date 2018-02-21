#!/usr/bin/bash

if [ -d "venv" ]; then
  echo "found python virtualenv"
else
  echo "creating python virtualenv"
  virtualenv venv
  source venv/bin/activate
  pip install -r requirements.txt
fi

source venv/bin/activate

$@

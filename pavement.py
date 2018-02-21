#!/usr/bin/env python

from paver.tasks import task
from paver.easy import sh
import os
import sys
sys.path.append( os.path.abspath('.') )

import helpers
version_git = helpers.get_version_from_git()

@task
def tasks():
   ''' List all paver tasks '''
   sh('paver --help |grep -A 8 "Tasks from pavement"')

@task
def build_python_client():
    ''' build the client '''
    if (os.path.isfile("config/semp-v2-swagger-config.yaml")):
        sh('rm -rf output/')
        sh('(cat config-python.json.template | sed "s/__VERSION__/{}/" > config.json && docker-compose run --rm openapi generate --config /editor/spec-files/config.json -l python -i /editor/spec-files/semp-v2-swagger-config.yaml -o /tmp/output/python && docker rmi solacesempclient_openapi -f)'.format(version_git, "solace_semp_client"))
    else:
        print("Please download latest OpenAPI spec YAML from sftp.solacesystems.com, rename it to 'default.yaml' and place them in solace_semp_client/")
        sys.exit(1)

@task
def build_java_client():
    ''' build the client '''
    if (os.path.isfile("config/semp-v2-swagger-config.yaml")):
        sh('rm -rf output/')
        sh('(cat config-java.json.template | sed "s/__VERSION__/{}/" > config.json && docker-compose run --rm openapi generate --config /editor/spec-files/config.json -l java -i /editor/spec-files/semp-v2-swagger-config.yaml -o /tmp/output/java && docker rmi solacesempclient_openapi -f)'.format(version_git, "solace_semp_client"))
        sh('(cd output/java && mvn deploy)')
    else:
        print("Please download latest OpenAPI spec YAML from sftp.solacesystems.com, rename it to 'default.yaml' and place them in solace_semp_client/")
        sys.exit(1)

@task
def integration_test_docker():
    ''' Run integration test in a solace docker container '''
    sh('( cd .docker && ./start.sh solace )')
    sh('nosetests --rednose --force-color test/integration/')
    sh('( cd .docker && docker stop solace )')

@task
def unit_test():
    ''' Run unit test '''
    sh('nosetests --rednose --force-color test/unit/')

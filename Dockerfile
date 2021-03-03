FROM swaggerapi/swagger-codegen-cli:2.4.13 as intermediate
WORKDIR /app
ARG sempver=none
ARG upload=0
COPY config config
COPY action-python.json.template /app/config/action-python.json
COPY config-python.json.template /app/config/config-python.json
COPY monitor-python.json.template /app/config/monitor-python.json
COPY swagger_templates swagger_templates
RUN sed -i "s/__VERSION__/$sempver/" /app/config/*-python.json
RUN mkdir output
RUN java -jar /opt/swagger-codegen-cli/swagger-codegen-cli.jar generate --config /app/config/config-python.json -o /app/output/python_config -i /app/config/${sempver}/semp-v2-swagger-config.yaml -l python -t /app/swagger_templates/python
RUN test -f config/${sempver}/semp-v2-swagger-action.yaml && java -jar /opt/swagger-codegen-cli/swagger-codegen-cli.jar generate --config /app/config/action-python.json -o /app/output/python_action -i /app/config/${sempver}/semp-v2-swagger-action.yaml -l python -t /app/swagger_templates/python || :
RUN test -f config/${sempver}/semp-v2-swagger-monitor.yaml && java -jar /opt/swagger-codegen-cli/swagger-codegen-cli.jar generate --config /app/config/monitor-python.json -o /app/output/python_monitor -i /app/config/${sempver}/semp-v2-swagger-monitor.yaml -l python -t /app/swagger_templates/python || :

FROM python:3-slim as pyintermediate
COPY --from=intermediate /app/output /app/output
WORKDIR /app/output
RUN mkdir tmp
RUN cd python_config; pip install -r requirements.txt; python setup.py bdist_wheel --universal; cp dist/*.whl ../tmp; cd ..
RUN ls
RUN test -d python_action && cd python_action; python setup.py bdist_wheel --universal; cp dist/*.whl ../tmp; cd .. || :
RUN test -d python_monitor && cd python_monitor; python setup.py bdist_wheel --universal; cp dist/*.whl ../tmp; cd .. || :

FROM ubuntu:20.04
RUN apt-get update && apt-get -y install python3-dev
RUN apt-get -y install python3-pip
WORKDIR /app
ARG sempver=none
ARG upload=0
COPY --from=pyintermediate /app/output/tmp/*.whl /tmp/
COPY .pypirc /root
RUN pwd
RUN echo $HOME
COPY requirements.txt requirements.txt
RUN python3 -m pip install --upgrade pip setuptools wheel twine
WORKDIR /tmp

RUN test "${upload}" -gt 0 && twine upload --repository pypi /tmp/solace_semp_action*.whl --verbose || echo "no upload arg passed"
RUN test "${upload}" -gt 0 && twine upload --repository pypi /tmp/solace_semp_config*.whl --verbose || echo "no upload arg passed"
RUN test "${upload}" -gt 0 && twine upload --repository pypi /tmp/solace_semp_monitor*.whl --verbose || echo "no upload arg passed"

ENTRYPOINT ["/bin/bash", "-c"]
FROM python:3.8 as builder
WORKDIR /usr/src/pysh
COPY . .
RUN pip install pipenv &&\
  pipenv install --deploy --ignore-pipfile &&\
  pipenv run build

FROM ubuntu:20.04
WORKDIR /usr/src/pysh
COPY --from=builder /usr/src/pysh/build/pysh /usr/bin
CMD [ "pysh" ]
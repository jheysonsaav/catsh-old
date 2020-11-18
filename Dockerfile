FROM python:3.9 as builder
WORKDIR /usr/src/catsh
COPY . .
RUN pip install pipenv &&\
  pipenv install --deploy --ignore-pipfile &&\
  pipenv run build

FROM ubuntu:20.04
WORKDIR /usr/src/pysh
COPY --from=builder /usr/src/catsh/build/catsh /usr/bin
CMD [ "catsh" ]
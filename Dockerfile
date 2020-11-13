FROM python:3.9 as builder
WORKDIR /usr/src/pysh
COPY . .
RUN pipenv install --deploy --ignore-pipfile && pipenv run build

FROM ubuntu:20.04
WORKDIR /usr/src/pysh
COPY --from=builder /usr/src/pysh/build/pysh /usr/bin
CMD [ "pysh" ]
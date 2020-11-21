# CatSh

## Install

to install catsh there are two ways which are the "native" way and the "interpreted" way.

### - Native compile

the first way (the native one) is where you manually compile pysh to a binary file with the pyinstaller tool

1. enter the virtual environment of python

```bash
pipenv shell
```

2. install all the necessary dependencies

```bash
pipenv install && pipenv install --dev
```

3. compile

```bash
pipenv run build
```

4. Start catsh!!

```bash
./dist/catsh # in linux or macos
dist/catsh.exe # in windows
```

### - Interpreted

In this method, catsh will be interpreted by python, therefore it is necessary to have installed a version of python higher than 3.6

> install from pypi

```bash
pip install catsh
```

> install from github

```bash
git clone https://github.com/JheysonDev/catsh.git
cd catsh
pip setup.py install
```

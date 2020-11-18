#!/usr/bin/env sh
python setup.py sdist bdist_wheel &&\
twine upload dist/*.tar.gz dist/*.whl &&\
rm -rf *.egg-info dist/*.tar.gz dist/*.whl
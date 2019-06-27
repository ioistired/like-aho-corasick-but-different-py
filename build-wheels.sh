#!/usr/bin/env sh

# Docker command: docker run --rm -v "$PWD:/io" quay.io/pypa/manylinux2010_x86_64 /io/build-wheels.sh

set -ex

curl -sSf https://sh.rustup.rs | sh -s -- -y
export PATH="$HOME/.cargo/bin:$PATH"

cd /io

for PYBIN in /opt/python/{cp36-cp36m,cp37-cp37m}/bin; do
    export PYTHON_SYS_EXECUTABLE="$PYBIN/python"

    "${PYBIN}/pip" install -U setuptools setuptools-rust
    "${PYBIN}/pip" wheel .; done

for whl in dist/*.whl; do
    auditwheel repair "$whl" -w dist/; done

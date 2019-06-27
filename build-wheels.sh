#!/usr/bin/env sh

# Docker command: docker run --rm -v "$PWD:/io" konstin2/pyo3-pack /io/build-wheels.sh

set -ex

curl -sSf https://sh.rustup.rs | sh -s -- -y
export PATH="$HOME/.cargo/bin:$PATH"

cd /io

for PYBIN in /opt/python/{cp36-cp36m,cp37-cp37m}/bin; do
    export PYTHON_SYS_EXECUTABLE="$PYBIN/python"

    "${PYBIN}/pip" install -U pyo3-pack
    build; done

for whl in target/wheels/*.whl; do
    auditwheel repair "$whl" -w target/wheels/; done

# gpapi-python

[![Build Linux & macOS wheels](https://github.com/iddaaz/rs-google-play-py/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/iddaaz/rs-google-play-py/actions/workflows/build.yml)

[![Checks](https://github.com/iddaaz/rs-google-play-py/actions/workflows/checks.yml/badge.svg)](https://github.com/iddaaz/rs-google-play-py/actions/workflows/checks.yml)

Python bindings for Google Play API. Based on and expands upon Electronic Frontier Foundation's [rs-google-play](https://github.com/EFForg/rs-google-play).

## Installation

### Option 1: Using pre-compiled wheels from our GitHub releases

You'll need to set up your `setup.py` to something like this:

```python
import sys
from setuptools import setup, find_packages

repo_url = "https://github.com/iddaaz/rs-google-play-py"

# obtaining the python interpreter's minor version
ver_minor = sys.version_info.minor

# for available arch-specific builds check out /releases @latest tag
arch = "manylinux_2_17_x86_64.manylinux2014_x86_64"

wheel_url = f"{repo_url}/releases/download/latest/gpapi_python-0.1.0-cp3{ver_minor}-cp3{ver_minor}-{arch}.whl"

setup(
    name="your-package",
    version="0.1.0",
    packages=find_packages(),
    install_requires=[
        f"gpapi_python @ {wheel_url}",
    ],
)
```

Then, you can install the package using `pip`:

```shell
pip install . # or pip install -e . for hot-reloading changes
```

### Option 2: Building from source

1. Clone the repo

   ```shell
   git clone https://github.com/iddaaz/rs-google-play-py.git
   ```

2. Set-up / Install dependencies

   ```shell
   cd rs-google-play-py/gpapi-python
   virtualenv venv
   source venv/bin/activate
   pip install maturin
   ```

3. Build and install the package

   ```shell
   maturin develop --release

   ```

## Usage

Please refer to [example.py](./gpapi-python/examples/example.py) for a complete example.

For an abstract example:

```python
# Get an instance
api = gpapi_python.GpApiWrapper(
    device,
    email,
)
# Login
api.set_aas_token(aas_token)
api.login()
# Obtain app details by package name
app_details = api.details(pkg_name)
# Initiate download
api.download(
    pkg_name,
    dst_path=dl_path,
    split_if_available=True,
    include_additional_files=True,
)
```

## Project Structure

- `googleplay-protobuf`: Protocol Buffer definitions
- `gpapi`: Core Rust implementation
- `gpapi-python`: Python bindings via PyO3

## License

Same as original [rs-google-play](https://github.com/EFForg/rs-google-play) project.

# gpapi-python

Python bindings for Google Play API. Based on and expands upon Electronic Frontier Foundation's [rs-google-play](https://github.com/EFForg/rs-google-play).

## Installation

> **Note:** Requires Python ≤ 3.12 due to PyO3/Maturin limitations

1. Clone the repo

   ```shell
   git clone https://github.com/iddaaz/rs-google-play-py.git
   ```

2. Set-up / Install dependencies

   ```shell
   cd rs-google-play-py/gpapi-python
   virtualenv -p python3.12 venv
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

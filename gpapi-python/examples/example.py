import logging
from pathlib import Path

import gpapi_python  # type: ignore I001
from yaml import safe_load

logging.basicConfig(level=logging.INFO)

with Path("config.yaml").open("r", encoding="utf-8") as f:
    config = safe_load(f)

pkg_name = "org.telegram.messenger"
email = config.get("auth", {}).get("email")
aas_token = config.get("auth", {}).get("aas_token")
device = config.get("app", {}).get("device_codename", "px_7a")
dl_path = config.get("app", {}).get("dl_path", "./temp_downloads")

api = gpapi_python.GpApiWrapper(
    device,
    email,
)
api.set_aas_token(aas_token)

if not Path(dl_path).exists():
    Path(dl_path).mkdir(parents=True)

api.login()
logging.info("Login successful")

details = api.details(pkg_name)
logging.info("Details: %s", details)

download_info = api.get_download_info(pkg_name, None)
logging.info("Download Info: %s", download_info)

api.download(
    pkg_name,
    dst_path=dl_path,
    split_if_available=True,
    include_additional_files=True,
)

logging.info("Operation completed.")

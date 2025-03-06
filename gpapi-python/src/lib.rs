// Python bindings for EFF's gpapi library
// (c) 2025 Cypho / @rvfet

use gpapi::Gpapi;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

#[pymodule]
fn gpapi_python(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[pyclass]
    struct GpApiWrapper {
        api: Gpapi,
        rt: Runtime,
    }

    #[pymethods]
    impl GpApiWrapper {
        #[new]
        fn new(device_codename: String, email: String) -> Self {
            GpApiWrapper {
                api: Gpapi::new(device_codename, email),
                rt: Runtime::new().unwrap(),
            }
        }

        fn set_aas_token(&mut self, aas_token: String) {
            self.api.set_aas_token(aas_token);
        }

        fn request_aas_token(&mut self, oauth_token: String) -> PyResult<()> {
            let future = self.api.request_aas_token(oauth_token);
            self.rt.block_on(future).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Error requesting AAS token: {}",
                    e
                ))
            })?;
            Ok(())
        }

        fn get_aas_token(&self) -> Option<String> {
            self.api.get_aas_token().map(|s| s.to_string())
        }

        fn login(&mut self) -> PyResult<()> {
            let future = self.api.login();
            self.rt.block_on(future).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Login failed: {}", e))
            })?;
            Ok(())
        }

        fn details(&self, pkg_name: String) -> PyResult<Option<String>> {
            let future = self.api.details(pkg_name);
            let details_result = self.rt.block_on(future).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Error getting details: {}",
                    e
                ))
            })?;
            Ok(details_result.map(|d| format!("{:?}", d)))
        }

        #[pyo3(signature = (pkg_name, version_code=None))]
        fn get_download_info(
            &self,
            pkg_name: String,
            version_code: Option<i32>,
        ) -> PyResult<Option<String>> {
            let future = self.api.get_download_info(pkg_name, version_code);
            let download_info = self.rt.block_on(future).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Error getting download info: {}",
                    e
                ))
            })?;

            let (main_apk, split_apks, additional_files) = download_info;
            let formatted = format!(
                "Main APK: {:?}\nSplit APKs: {:?}\nAdditional Files: {:?}",
                main_apk, split_apks, additional_files
            );
            Ok(Some(formatted))
        }

        #[pyo3(signature = (pkg_name, version_code=None, *, split_if_available, include_additional_files, dst_path))]
        fn download(
            &self,
            pkg_name: String,
            version_code: Option<i32>,
            split_if_available: bool,
            include_additional_files: bool,
            dst_path: String,
        ) -> PyResult<Option<String>> {
            let path = std::path::Path::new(&dst_path);
            let future = self.api.download(
                pkg_name,
                version_code,
                split_if_available,
                include_additional_files,
                path,
                None,
            );
            let download_result = self.rt.block_on(future).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Download error: {}", e))
            })?;
            Ok(Some(format!("{:?}", download_result)))
        }
    }
    m.add_class::<GpApiWrapper>()?;
    Ok(())
}

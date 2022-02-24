use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};
use pythonize::{pythonize};
use serde_json::{from_str, Value};
use std::fs;


///
/// The strategy to apply the steady-state hypothesis of the experiment
/// is an enum in Chaos Toolkit, rather than a mere string. So need to mimic
/// we send such an enum to chaoslib so that it goes through.
/// 
#[pyclass]
struct Strategy {
    val: &'static str
}

#[pymethods]
impl Strategy {
    #[classattr]
    const BEFORE_METHOD: &'static str = "before-method-only";
    const AFTER_METHOD: &'static str = "after-method-only";
    const DURING_METHOD: &'static str = "during-method-only";
    const DEFAULT: &'static str = "default";
    const CONTINUOUS: &'static str = "continuous";

    #[getter(value)]
    fn get_value(&self) -> PyResult<&'static str> {
        Ok(self.val)
    }
}

///
/// Configure the Chaos Toolkit logger on the Python side
/// 
fn configure_logger(py: Python) -> Result<(), PyErr> {
    let md = py.import("chaostoolkit.logging")?;
    let fun = md.getattr("configure_logger")?;
    fun.call0()?;
    return Ok(());
}

///
/// Load the experiment file as a JSON string then turns into a Python
/// object that can be sent down the run experiment function
/// 
fn load_experiment(py: Python, experiment_path: String) -> Result<PyObject, PyErr> {
    let data = fs::read_to_string(experiment_path).unwrap();
    let res: Value = from_str(&data).unwrap();
    return Ok(pythonize(py, &res).unwrap());
}

///
/// Run the experiment in the Python world
/// 
fn run_experiment(py: Python, experiment: PyObject) -> Result<&PyAny, PyErr> {
    let settings: &PyDict = PyDict::new(py);
    let strategy: Py<Strategy> = Py::new(py, Strategy{ val: Strategy::DEFAULT })?;
    let kwargs = vec![
        ("experiment", experiment),
        ("settings", settings.to_object(py)),
        ("strategy", strategy.into_py(py)),
        ("schedule", py.None()),
        ("experiment_vars", py.None())
    ].into_py_dict(py);

    let md = py.import("chaoslib.experiment")?;
    let fun = md.getattr("run_experiment")?;
    let journal = fun.call((), Some(kwargs))?;

    return Ok(journal);
}

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        configure_logger(py).unwrap();
        let experiment: PyObject = load_experiment(py, "./experiment.json".to_string())?;
        let journal = run_experiment(py, experiment)?;
        Ok(())
    })
}

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

#[derive(Debug)]
pub struct CfscrapeData {
    pub cookies: String,
    pub user_agent: String,
}

#[derive(Debug)]
pub enum Error {
    Python(pyo3::PyErr),
}

impl From<PyErr> for Error {
    fn from(err: PyErr) -> Self {
        Error::Python(err)
    }
}

static PY_CODE: &str = "cfscrape.get_cookie_string(url, user_agent)";

pub fn get_cookie_string(url: &str, user_agent: Option<&str>) -> Result<CfscrapeData, Error> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let cfscrape: PyObject = py.import("cfscrape")?.into_py(py);

    let url: PyObject = url.into_py(py);
    let user_agent = user_agent.into_py(py);

    let locals = [
        ("url", url),
        ("user_agent", user_agent),
        ("cfscrape", cfscrape),
    ]
    .into_py_dict(py);

    let (cookies, user_agent): (String, String) =
        py.eval(PY_CODE, None, Some(&locals))?.extract()?;

    Ok(CfscrapeData {
        cookies,
        user_agent,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let domains = ["https://www.furaffinity.net", "https://www.google.com"];
        let user_agents = [None, Some("rust-cfscrape test")];

        for domain in domains.iter() {
            for user_agent in user_agents.iter() {
                let data = match get_cookie_string(domain, *user_agent) {
                    Ok(data) => data,
                    Err(err) => panic!("error: {:?}", err),
                };

                if let Some(agent) = user_agent {
                    assert_eq!(data.user_agent, *agent, "user agent differed from provided");
                }
            }
        }
    }
}

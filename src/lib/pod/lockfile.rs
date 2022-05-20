use std::collections::{HashMap, HashSet};
use std::path::Path;
use regex;
use yaml_rust::{YamlLoader, Yaml};
use anyhow::{anyhow};
use regex::Regex;

pub(crate) struct PodItem {
    #[allow(dead_code)]
    pub(crate) name: String,
    pub(crate) vers: Option<String>,
    pub(crate) parents: HashSet<String>,
    pub(crate) children: HashSet<String>,
}

impl PodItem {
    fn new(name: String, vers: Option<String>) -> Self {
        PodItem { name, vers, parents: HashSet::new(), children: HashSet::new() }
    }
}

pub(crate) struct ExternalSource {
    pub(crate) name: String,
    pub(crate) path: Option<String>,
}

mod parse_failed {
    pub(super) const UNEXPECTED_TYPE: &'static str = "unexpected type";
    pub(super) const UNEXPECTED_TEXT: &'static str = "unexpected text";
    pub(super) const UNEXPECTED_LENGTH: &'static str = "unexpected length";
}

// 分析 Podfile.lock 结构
// 参考 cocoapods-core

pub(crate) struct Lockfile {
    root: Yaml,
}

impl Lockfile {
    pub(crate) fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let docs = YamlLoader::load_from_str(content.as_str())?;
        if docs.len() < 1 {
            Err(anyhow!("docs should not be empty"))
        } else {
            Ok(Lockfile { root: docs[0].clone() })
        }
    }

    // 解析 PODS 里的内容
    pub(crate) fn pods(&self) -> anyhow::Result<HashMap<String, PodItem>> {
        let name_and_vers_from_str = |s: &str| -> anyhow::Result<(String, Option<String>)> {
            let (name, vers) = ("name", "vers");
            let raw_str = format!("(?P<{}>(?:\\s?[^\\s(])+)(?: \\((?P<{}>.+)\\))?", name, vers);
            let re = Regex::new(raw_str.as_str()).unwrap();
            let caps = re.captures(s)
                .ok_or(anyhow!(parse_failed::UNEXPECTED_TEXT))?;
            let name = caps.name(name)
                .ok_or(anyhow!(parse_failed::UNEXPECTED_TEXT))?
                .as_str()
                .to_owned();
            let vers = caps.name(vers)
                .map(|v| v.as_str().to_owned());
            Ok((name, vers))
        };

        let dep_name_from_str = |s: &str| -> anyhow::Result<String> {
            let name = "name";
            let raw_str = format!("(?P<{}>(?:\\s?[^\\s(])+)(?: \\((.+)\\))?", name);
            let re = Regex::new(raw_str.as_str()).unwrap();
            let caps = re.captures(s)
                .ok_or(anyhow!(parse_failed::UNEXPECTED_TEXT))?;
            caps.name(name)
                .map(|m| m.as_str().to_owned())
                .ok_or(anyhow!(parse_failed::UNEXPECTED_TEXT))
        };

        let pods = &self.root["PODS"];
        let mut result = HashMap::new();
        let vec = pods.as_vec().ok_or(anyhow!(parse_failed::UNEXPECTED_TYPE))?;
        for v in vec {
            let (pod_line, dep_lines) =
                match v {
                    Yaml::String(s) => {
                        (s.as_str(), vec![])
                    },
                    Yaml::Hash(h) => {
                        if h.len() != 1 {
                            return Err(anyhow!(parse_failed::UNEXPECTED_LENGTH))
                        }
                        let k = h.keys().last().unwrap();
                        let name = k.as_str().ok_or(anyhow!(parse_failed::UNEXPECTED_TYPE))?;
                        let deps = h.get(k).unwrap().as_vec().ok_or(anyhow!(parse_failed::UNEXPECTED_TYPE))?;
                        let mut dep_lines = Vec::<&str>::new();
                        for dep in deps {
                            let line = dep.as_str().ok_or(anyhow!(parse_failed::UNEXPECTED_TYPE))?;
                            dep_lines.push(line);
                        }
                        (name, dep_lines)
                    },
                    _ => {
                        return Err(anyhow!(parse_failed::UNEXPECTED_TYPE))
                    }
                };
            let (pod_name, pod_vers) = name_and_vers_from_str(pod_line)?;
            if !result.contains_key(&pod_name) {
                let item = PodItem::new(pod_name.clone(), pod_vers);
                result.insert(pod_name.clone(), item);
            } else {
                let mut item = result.get_mut(&pod_name).unwrap();
                item.vers = pod_vers;
            };
            for line in dep_lines {
                let dep_name = dep_name_from_str(line)?;
                if dep_name != pod_name {
                    let pod = result.get_mut(&pod_name).unwrap();
                    pod.children.insert(dep_name.clone());
                }
                let dep =
                    if !result.contains_key(&dep_name) {
                        let item = PodItem::new(dep_name.clone(), None);
                        result.insert(dep_name.clone(), item);
                        result.get_mut(&dep_name).unwrap()
                    } else {
                        result.get_mut(&dep_name).unwrap()
                    };
                if dep_name != pod_name {
                    dep.parents.insert(pod_name.clone());
                }
            }
        }
        Ok(result)
    }

    // 解析 EXTERNAL SOURCES 里的内容
    #[allow(dead_code)]
    pub(crate) fn external_sources(&self) -> anyhow::Result<HashMap<String, ExternalSource>> {
        let sources = &self.root["EXTERNAL SOURCES"];
        let hash = sources.as_hash().ok_or(anyhow!(parse_failed::UNEXPECTED_TYPE))?;
        let mut ret = HashMap::new();
        for (key, value) in hash {
            let name = key.as_str().ok_or(anyhow!(parse_failed::UNEXPECTED_TYPE))?.to_owned();
            let mut es = ExternalSource { name, path: None };
            if let Some(path) = value[":path"].as_str() {
                es.path = Some(path.to_owned());
            }
            ret.insert(es.name.clone(), es);
        }
        Ok(ret)
    }

}
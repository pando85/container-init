/// ANCHOR: module
/// # set_vars
///
/// This module allows setting new variables.
///
/// ## Parameters
///
/// ```yaml
/// key_value:
///   type: list
///   required: true
///   description: |
///     This module takes key/value pairs and save un current Vars context.
/// ```
///
/// ## Example
///
/// ```yaml
/// - set_vars:
///     foo: boo
///
/// - assert
///     that:
///       - foo == 'boo'
/// ```
/// ANCHOR_END: module
use crate::error::{Error, ErrorKind, Result};
use crate::modules::ModuleResult;
use crate::vars::Vars;

use serde_yaml::Value;
use yaml_rust::{Yaml, YamlEmitter};

pub fn exec(params: Yaml, vars: Vars) -> Result<(ModuleResult, Vars)> {
    let mut new_vars = vars;

    params
        .as_hash()
        .ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!("{:?} must be a dict", &params),
            )
        })
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?
        .iter()
        .map(|hash_map| {
            let mut yaml_str = String::new();
            let mut emitter = YamlEmitter::new(&mut yaml_str);
            emitter
                .dump(hash_map.1)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            let yaml: Value = serde_yaml::from_str(&yaml_str)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

            new_vars.insert(
                hash_map.0.as_str().ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!("{:?} is not a valid string", &hash_map.0),
                    )
                })?,
                &yaml,
            );
            Ok(new_vars.clone())
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((
        ModuleResult {
            changed: false,
            output: None,
            extra: None,
        },
        new_vars,
    ))
}

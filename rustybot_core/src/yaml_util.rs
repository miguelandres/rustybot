// Copyright (c) 2021-2021 Miguel Barreto and others
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

extern crate yaml_rust;

use crate::directive::{Setting, Settings};
use yaml_rust::Yaml;

pub fn get_boolean_setting(
    name: &str,
    context_settings: &Settings,
    directive_defaults: &Settings,
) -> Result<bool, String> {
    if let Setting::Boolean(b) = get_setting(name, context_settings, directive_defaults)? {
        Ok(b)
    } else {
        Err(format!("Setting {} was found but is not boolean", name))
    }
}

pub fn get_setting(
    name: &str,
    context_settings: &Settings,
    directive_defaults: &Settings,
) -> Result<Setting, String> {
    if let Some(setting) = context_settings.get(name) {
        Ok(setting.clone())
    } else if let Some(setting) = directive_defaults.get(name) {
        Ok(setting.clone())
    } else {
        Err(format!(
            "Setting {} couldn't be found in context or defaults",
            name
        ))
    }
}

pub fn get_string_content_or_keyed_value(yaml: &Yaml, key: Option<&str>) -> Result<String, String> {
    match (yaml, key) {
        (Yaml::String(s), _) => Ok(s.clone()),
        (Yaml::Hash(hash), Some(key)) => {
            match hash.get(&Yaml::String(String::from(key))).ok_or(format!(
                "Yaml hash does not contain key {}: {:?}",
                key, hash
            ))? {
                Yaml::String(s) => Ok(s.clone()),
                yaml => Err(format!(
                    "Yaml Hash contains key {} but its value is not a string: {:?}",
                    key, yaml
                )),
            }
        }
        (yaml, Some(key)) => Err(format!(
            "Yaml value is not a string or does not contain key {}: {:?}",
            key, yaml
        )),
        (yaml, None) => Err(format!("Yaml value is not a string: {:?}", yaml)),
    }
}

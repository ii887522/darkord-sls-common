use crate::constants;
use optarg2chain::optarg_impl;
use serde_json::{json, Map, Value};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct SensitiveData<'a> {
    hidden_value: Map<String, Value>,
    shown_value: Map<String, Value>,
    extra_sensitive_keys: &'a [&'a str],
}

#[derive(Debug, PartialEq)]
struct SensitiveDataMut<'a> {
    hidden_value: &'a mut Map<String, Value>,
    shown_value: &'a mut Map<String, Value>,
}

#[derive(Debug, PartialEq)]
struct OptionalSensitiveDataMut<'a> {
    hidden_value: Option<&'a mut Map<String, Value>>,
    shown_value: Option<&'a mut Map<String, Value>>,
}

#[optarg_impl]
impl<'a> SensitiveData<'a> {
    #[optarg_method(SensitiveDataNewBuilder, call)]
    pub fn new(
        value: Map<String, Value>,
        #[optarg_default] extra_sensitive_keys: &'a [&'a str],
    ) -> Self {
        Self {
            hidden_value: Map::with_capacity(value.len()),
            shown_value: value,
            extra_sensitive_keys,
        }
    }

    pub const fn get(&self) -> &Map<String, Value> {
        &self.shown_value
    }

    pub fn into_data(self) -> Map<String, Value> {
        self.shown_value
    }

    pub fn hide(&mut self) {
        // A stack of references to the sensitive data to be processed by the below logic
        let mut sensitive_data_stack = vec![SensitiveDataMut {
            hidden_value: &mut self.hidden_value,
            shown_value: &mut self.shown_value,
        }];

        while let Some(sensitive_data) = sensitive_data_stack.pop() {
            // Move sensitive entries from shown_value to hidden_value
            for &sensitive_key in constants::SENSITIVE_KEYS
                .iter()
                .chain(self.extra_sensitive_keys)
            {
                if let Some((sensitive_key, sensitive_value)) =
                    sensitive_data.shown_value.remove_entry(sensitive_key)
                {
                    sensitive_data
                        .hidden_value
                        .insert(sensitive_key, sensitive_value);
                }
            }

            let mut sensitive_data_map = HashMap::with_capacity(sensitive_data.shown_value.len());

            for (k, v) in sensitive_data.shown_value {
                let Value::Object(sv) = v else {
                    continue;
                };

                sensitive_data
                    .hidden_value
                    .insert(k.to_string(), json!(Map::with_capacity(sv.len())));

                let v = OptionalSensitiveDataMut {
                    hidden_value: None,
                    shown_value: Some(sv),
                };

                sensitive_data_map.insert(k, v);
            }

            for (k, v) in sensitive_data.hidden_value {
                sensitive_data_map.entry(k).and_modify(|sensitive_data| {
                    sensitive_data.hidden_value = Some(v.as_object_mut().unwrap())
                });
            }

            // Queue up the rest of entries to be processed in the next iteration
            // Will be processed in LIFO order because using the stack
            let sensitive_data_iter =
                sensitive_data_map
                    .into_values()
                    .map(|sensitive_data| SensitiveDataMut {
                        hidden_value: sensitive_data.hidden_value.unwrap(),
                        shown_value: sensitive_data.shown_value.unwrap(),
                    });

            sensitive_data_stack.extend(sensitive_data_iter);
        }
    }

    pub fn show(&mut self) {
        // A stack of references to the sensitive data to be processed by the below logic
        let mut sensitive_data_stack = vec![SensitiveDataMut {
            hidden_value: &mut self.hidden_value,
            shown_value: &mut self.shown_value,
        }];

        while let Some(sensitive_data) = sensitive_data_stack.pop() {
            // Move sensitive entries from hidden_value to shown_value
            for (k, v) in sensitive_data.hidden_value.iter_mut() {
                sensitive_data
                    .shown_value
                    .entry(k)
                    .or_insert_with(|| v.take());
            }

            let mut sensitive_data_map = HashMap::with_capacity(sensitive_data.hidden_value.len());

            for (k, v) in sensitive_data.hidden_value {
                let Value::Object(hv) = v else {
                    continue;
                };

                let v = OptionalSensitiveDataMut {
                    hidden_value: Some(hv),
                    shown_value: None,
                };

                sensitive_data_map.insert(k, v);
            }

            for (k, v) in sensitive_data.shown_value {
                sensitive_data_map.entry(k).and_modify(|sensitive_data| {
                    sensitive_data.shown_value = Some(v.as_object_mut().unwrap());
                });
            }

            // Queue up the rest of entries to be processed in the next iteration
            // Will be processed in LIFO order because using the stack
            let sensitive_data_iter =
                sensitive_data_map
                    .into_values()
                    .map(|sensitive_data| SensitiveDataMut {
                        hidden_value: sensitive_data.hidden_value.unwrap(),
                        shown_value: sensitive_data.shown_value.unwrap(),
                    });

            sensitive_data_stack.extend(sensitive_data_iter);
        }
    }
}

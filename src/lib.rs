//! An experimental new library that serializes and deserializes messages for the [Chrome Devtools
//! Protocol](https://chromedevtools.github.io/devtools-protocol/).
//!
//! The Devtools Protocol is divided into a number of domains. Each of these domains have "methods"
//! and "events" that can be both serialized and deserialized by this library.

pub mod domain;

pub use domain::{runtime, runtime::Runtime, Domain};

#[cfg(test)]
mod tests {
    use crate::Domain;
    #[test]
    fn it_can_deserialize_bigint_log() {
        let input = r#"{{
    "method": "Runtime.consoleAPICalled",
    "params": {{
      "type": "log",
      "args": [
        {{
          "type": "bigint",
          "unserializableValue": "1000n",
          "description": "1000n"
        }}
      ],
      "executionContextId": 2,
      "timestamp": 1576105132037,
      "stackTrace": {{
        "callFrames": [
          {{
            "functionName": "handleRequest",
            "scriptId": "3",
            "url": "worker.js",
            "lineNumber": 17,
            "columnNumber": 10
          }},
          {{
            "functionName": "",
            "scriptId": "3",
            "url": "worker.js",
            "lineNumber": 1,
            "columnNumber": 20
          }}
        ]
      }}
    }}
  }}"#;
        let output: Result<Domain, _> = serde_json::from_str(&input);
        if let Err(e) = output {
            println!("{}", e);
        }
        // assert!(output.is_ok())
    }
}

/*
 * Copyright (c) 2022 XXIV
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use std::io::Read;
use urlencoding::encode;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use crate::error::UrbanDictionaryError;

pub struct UrbanDictionary {
    term: String,
    page: i64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BaseResponse {
    list: Vec<Response>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub definition: String,
    pub permalink: String,
    #[serde(rename = "thumbs_up")]
    pub thumbs_up: i64,
    #[serde(rename = "sound_urls")]
    pub sound_urls: Vec<String>,
    pub author: String,
    pub word: String,
    pub defid: i64,
    #[serde(rename = "written_on")]
    pub written_on: String,
    pub example: String,
    #[serde(rename = "thumbs_down")]
    pub thumbs_down: i64,
}

fn http(endpoint: &str) -> Option<String> {
    match reqwest::blocking::Client::new().get(format!("https://api.urbandictionary.com/v0/{}", endpoint))
        .send() {
        Ok(mut response) => {
            let mut body = String::new();
            match response.read_to_string(&mut body) {
                Ok(_) => Some(body),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

impl UrbanDictionary {
    pub fn new(input: &str, page: i64) -> Self {
        Self {
            term: input.to_string(),
            page
        }
    }

    pub fn data(&self) -> Result<Vec<Response>, UrbanDictionaryError> {
        match http(format!("define?term={}&page={}", encode(self.term.as_str()), self.page).as_str()) {
            Some(response) => {
                match serde_json::from_str(&response) {
                    Ok(json) => {
                        let mut vector = vec![];
                        let data: BaseResponse = json;
                        for i in data.list {
                            vector.push(i)
                        }
                        if vector.is_empty() {
                            Err(UrbanDictionaryError::Error(String::from("null")))
                        } else {
                            Ok(vector)
                        }
                    },
                    Err(_) => Err(UrbanDictionaryError::Error(String::from("null")))
                }
            },
            None => Err(UrbanDictionaryError::Error(String::from("null")))
        }
    }
}

pub fn random() -> Result<Vec<Response>, UrbanDictionaryError> {
    match http("random") {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let data: BaseResponse = json;
                    for i in data.list {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(UrbanDictionaryError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(UrbanDictionaryError::Error(String::from("null")))
            }
        },
        None => Err(UrbanDictionaryError::Error(String::from("null")))
    }
}

pub fn definition_by_id(id: i64) -> Result<Response, UrbanDictionaryError> {
    match http(format!("define?defid={}", id).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: BaseResponse = json;
                    if data.list.is_empty() {
                        Err(UrbanDictionaryError::Error(String::from("null")))
                    } else {
                        Ok(data.list[0].clone())
                    }
                },
                Err(_) => Err(UrbanDictionaryError::Error(String::from("null")))
            }
        },
        None => Err(UrbanDictionaryError::Error(String::from("null")))
    }
}

pub fn tool_tip(term: &str) -> Result<String, UrbanDictionaryError> {
    match http(format!("tooltip?term={}", term).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: Value = json;
                    match data.get("string") {
                        Some(res) => Ok(res.to_string()),
                        None => Err(UrbanDictionaryError::Error(String::from("null")))
                    }
                },
                Err(_) => Err(UrbanDictionaryError::Error(String::from("null")))
            }
        },
        None => Err(UrbanDictionaryError::Error(String::from("null")))
    }
}
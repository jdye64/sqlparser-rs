// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![warn(clippy::all)]
//! Test SQL syntax specific to Dask-SQL. The parser based on the
//! generic dialect is also tested (on the inputs it can handle).

#[macro_use]
mod test_utils;
use test_utils::*;

use sqlparser::ast::*;
use sqlparser::dialect::{DaskDialect};

#[test]
fn test_create_model_with() {
    // let sql = "CREATE MODEL my_model WITH (
    //     model_class = 'sklearn.ensemble.GradientBoostingClassifier',
    //     wrap_predict = True,
    //     target_column = 'target'
    // )"; 
    let sql = "CREATE MODEL my_model WITH (model_class = 'sklearn.ensemble.GradientBoostingClassifier')";
    match dask().verified_stmt(sql) {
        Statement::CreateModel { model_name, model_class } => {
            assert_eq!(model_name.to_string(), "my_model")
        },
        _ => panic!("fail")
    }
}

fn dask() -> TestedDialects {
    TestedDialects {
        dialects: vec![Box::new(DaskDialect {})],
    }
}

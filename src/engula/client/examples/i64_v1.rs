// Copyright 2022 The Engula Authors.
//
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

use anyhow::Result;
use engula_client::v1::{Universe, I64};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://localhost:21716";
    let uv = Universe::connect(url).await?;
    let db = uv.create_database("i64").await?;
    let co = db.create_collection("i64").await?;

    co.set("a", 1).await?;
    let a: i64 = co.get("a").await?;
    println!("a = {:?}", a);

    co.mutate("a", I64::add(2)).await?;
    let a: i64 = co.get("a").await?;
    println!("a.add(2) = {:?}", a);

    co.mutate("a", I64::sub(3)).await?;
    let a: i64 = co.get("a").await?;
    println!("a.sub(3) = {:?}", a);

    let mut txn = co.begin();
    txn.mutate("a", I64::add(1));
    txn.mutate("b", I64::sub(2));
    txn.commit().await?;
    println!("a = {:?}", co.get("a").await?);
    println!("b = {:?}", co.get("b").await?);

    Ok(())
}

// I64: get,set,delete,add,sub
// Blob: get,len,range,set,delete,pop_back,pop

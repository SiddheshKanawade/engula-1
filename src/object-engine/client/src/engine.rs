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

use object_engine_master::proto::*;

use crate::{Env, Result, Tenant};

#[derive(Clone)]
pub struct Engine<E: Env> {
    env: E,
}

impl<E: Env> Engine<E> {
    pub async fn open(env: E) -> Result<Self> {
        Ok(Self { env })
    }

    pub async fn tenant(&self, name: &str) -> Result<Tenant<E>> {
        let tenant = self.env.tenant(name).await?;
        Ok(Tenant::new(self.env.clone(), tenant))
    }

    pub async fn create_tenant(&self, name: &str) -> Result<Tenant<E>> {
        let req = CreateTenantRequest {
            name: name.to_owned(),
            ..Default::default()
        };
        let req = request_union::Request::CreateTenant(req);
        self.env.handle_union(req).await?;
        self.tenant(name).await
    }
}

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

use std::ops::RangeBounds;

use engula_apis::v1::*;

use super::{call, MutateExpr, SelectExpr};

pub struct Map(MapValue);

impl From<Map> for Value {
    fn from(v: Map) -> Self {
        v.0.into()
    }
}

impl Map {
    pub fn new(value: impl Into<MapValue>) -> Self {
        Self(value.into())
    }

    pub fn len() -> MapSelect {
        MapSelect::len()
    }

    pub fn field(field: impl Into<ListValue>) -> MapSelect {
        MapSelect::field(field)
    }

    pub fn range<T>(range: impl RangeBounds<T>) -> MapSelect
    where
        T: Clone + Into<Value>,
    {
        MapSelect::range(range)
    }

    pub fn extend(value: impl Into<MapValue>) -> MapMutate {
        MapMutate::extend(value)
    }

    pub fn delete(index: impl Into<ListValue>) -> MapMutate {
        MapMutate::delete(index)
    }
}

pub struct MapSelect {
    expr: MapExpr,
}

impl MapSelect {
    fn new(call: CallExpr) -> Self {
        Self {
            expr: MapExpr { call: Some(call) },
        }
    }

    pub fn len() -> Self {
        Self::new(call::len())
    }

    pub fn field(field: impl Into<ListValue>) -> Self {
        Self::new(call::get_index(field.into()))
    }

    pub fn range<T>(range: impl RangeBounds<T>) -> Self
    where
        T: Clone + Into<Value>,
    {
        Self::new(call::get_range(call::range(range)))
    }
}

impl From<MapSelect> for SelectExpr {
    fn from(v: MapSelect) -> Self {
        Expr::from(v.expr).into()
    }
}

pub struct MapMutate {
    expr: MapExpr,
}

impl MapMutate {
    fn new(call: CallExpr) -> Self {
        Self {
            expr: MapExpr { call: Some(call) },
        }
    }

    pub fn extend(value: impl Into<MapValue>) -> Self {
        Self::new(call::extend(value.into()))
    }

    pub fn delete(field: impl Into<ListValue>) -> Self {
        Self::new(call::delete_index(field.into()))
    }
}

impl From<MapMutate> for MutateExpr {
    fn from(v: MapMutate) -> Self {
        Expr::from(v.expr).into()
    }
}

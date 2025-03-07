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

use std::ops::{Bound, RangeBounds};

use engula_apis::v1::*;

macro_rules! call {
    ($func:expr) => {
        CallExpr {
            func: $func as i32,
            args: vec![],
            ..Default::default()
        }
    };
    ($func:expr, $arg0:expr) => {
        CallExpr {
            func: $func as i32,
            args: vec![$arg0.into()],
            ..Default::default()
        }
    };
}

macro_rules! index_call {
    ($func:expr, $index:expr) => {
        CallExpr {
            func: $func as i32,
            args: vec![],
            operand: Some(call_expr::Operand::Index($index.into())),
        }
    };
}

macro_rules! range_call {
    ($func:expr, $range:expr) => {
        CallExpr {
            func: $func as i32,
            args: vec![],
            operand: Some(call_expr::Operand::Range($range.into())),
        }
    };
}

pub fn get() -> CallExpr {
    call!(Function::Get)
}

pub fn get_index(i: impl Into<TypedValue>) -> CallExpr {
    index_call!(Function::Get, i)
}

pub fn get_range(r: impl Into<TypedRange>) -> CallExpr {
    range_call!(Function::Get, r)
}

pub fn set(v: impl Into<TypedValue>) -> CallExpr {
    call!(Function::Set, v)
}

pub fn delete() -> CallExpr {
    call!(Function::Delete)
}

pub fn delete_index(i: impl Into<TypedValue>) -> CallExpr {
    index_call!(Function::Delete, i)
}

pub fn add(v: impl Into<TypedValue>) -> CallExpr {
    call!(Function::Add, v)
}

pub fn sub(v: impl Into<TypedValue>) -> CallExpr {
    call!(Function::Sub, v)
}

pub fn trim(r: impl Into<TypedRange>) -> CallExpr {
    range_call!(Function::Trim, r)
}

pub fn lpop(n: impl Into<TypedValue>) -> CallExpr {
    call!(Function::Lpop, n)
}

pub fn rpop(n: impl Into<TypedValue>) -> CallExpr {
    call!(Function::Rpop, n)
}

pub fn lpush(v: impl Into<TypedValue>) -> CallExpr {
    call!(Function::Lpush, v)
}

pub fn rpush(v: impl Into<TypedValue>) -> CallExpr {
    call!(Function::Rpush, v)
}

pub fn len() -> CallExpr {
    call!(Function::Len)
}

pub fn extend(v: impl Into<TypedValue>) -> CallExpr {
    call!(Function::Extend, v)
}

pub fn range<T>(r: impl RangeBounds<T>) -> TypedRange
where
    T: Clone + Into<TypedValue>,
{
    let mut expr = TypedRange::default();
    match r.start_bound().cloned() {
        Bound::Included(start) => {
            expr.start = Some(start.into());
            expr.start_bound = RangeBound::Included as i32;
        }
        Bound::Excluded(start) => {
            expr.start = Some(start.into());
            expr.start_bound = RangeBound::Excluded as i32;
        }
        Bound::Unbounded => {
            expr.start_bound = RangeBound::Unbounded as i32;
        }
    }
    match r.end_bound().cloned() {
        Bound::Included(end) => {
            expr.end = Some(end.into());
            expr.end_bound = RangeBound::Included as i32;
        }
        Bound::Excluded(end) => {
            expr.end = Some(end.into());
            expr.end_bound = RangeBound::Excluded as i32;
        }
        Bound::Unbounded => {
            expr.end_bound = RangeBound::Unbounded as i32;
        }
    }
    expr
}

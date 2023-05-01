// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::programmable_transactions::{
    context::{load_type, new_session},
    linkage_view::{LinkageInfo, LinkageView},
    types::StorageView,
};
use move_core_types::value::{MoveStructLayout, MoveTypeLayout};
use move_vm_runtime::{move_vm::MoveVM, session::Session};
use std::{collections::BTreeMap, sync::Arc};
use move_core_types::language_storage::{StructTag, TypeTag};
use sui_protocol_config::ProtocolConfig;
use sui_types::{
    error::SuiError,
    layout_resolver::LayoutResolver,
    metrics::LimitsMetrics,
    object::{MoveObject, ObjectFormatOptions},
};

pub(crate) struct TypeLayoutResolver<'state, 'vm, S: StorageView> {
    session: Session<'state, 'vm, LinkageView<'state, S>>,
}

impl<'state, 'vm, S: StorageView> TypeLayoutResolver<'state, 'vm, S> {
    pub(crate) fn new(
        vm: &'vm MoveVM,
        state_view: &'state S,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Self {
        let session = new_session(
            vm,
            LinkageView::new(state_view, LinkageInfo::Unset),
            BTreeMap::new(),
            false,
            protocol_config,
            metrics.clone(),
        );
        Self { session }
    }
}

impl<'state, 'vm, S: StorageView> LayoutResolver for TypeLayoutResolver<'state, 'vm, S> {
    fn get_layout(
        &mut self,
        object: &MoveObject,
        format: ObjectFormatOptions
    ) -> Result<MoveStructLayout, SuiError> {
        let struct_tag: StructTag = object.type_().clone().into();
        let type_tag: TypeTag = TypeTag::from(struct_tag.clone());
        let ty = match load_type(&mut self.session, &type_tag) {
            Err(_) => return Err(SuiError::FailObjectLayout { st: format!("{}", struct_tag) }),
            Ok(ty) => ty,
        };
        let layout = if format.include_types() {
            self.session.type_to_fully_annotated_layout(&ty)
        } else {
            self.session.type_to_type_layout(&ty)
        };
        match layout {
            Err(_) => Err(SuiError::FailObjectLayout { st: format!("{}", struct_tag) }),
            Ok(type_layout) => match type_layout {
                MoveTypeLayout::Struct(layout) => Ok(layout),
                _ => Err(SuiError::FailObjectLayout { st: format!("{}", struct_tag) }),
            },
        }
    }
}

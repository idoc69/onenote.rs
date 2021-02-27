use crate::errors::{ErrorKind, Result};
use crate::fsshttpb::data::exguid::ExGuid;
use crate::one::property::object_reference::ObjectReference;
use crate::one::property::PropertyType;
use crate::one::property_set::PropertySetId;
use crate::onestore::object::Object;

#[derive(Debug)]
pub(crate) struct Data {
    pub(crate) lines: Vec<ExGuid>,
}

pub(crate) fn parse(object: &Object) -> Result<Data> {
    if object.id() != PropertySetId::InkAnalysisParagraph.as_jcid() {
        return Err(ErrorKind::MalformedOneNoteFileData(
            format!("unexpected object type: 0x{:X}", object.id().0).into(),
        )
        .into());
    }

    let lines =
        ObjectReference::parse_vec(PropertyType::InkAnalysisReference, object)?.unwrap_or_default();

    Ok(Data { lines })
}

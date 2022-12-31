//! Some XML helpers

use crate::{error::Error, Uuid};
use xml::{writer::XmlEvent, EventWriter};

/// A trait for untagged primitives that can be written to an XML document
pub trait XmlWritePrimitive {
    /// Writes `self` as XML element to the writer
    fn write(&self, tag: &str, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error>;
}
impl XmlWritePrimitive for String {
    fn write(&self, tag: &str, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error> {
        // Serialize the tag
        let tag_start = XmlEvent::start_element(tag);
        writer.write(tag_start)?;

        // Write the value
        let value = XmlEvent::characters(self);
        writer.write(value)?;

        // Close element
        let tag_end = XmlEvent::end_element().name(tag);
        writer.write(tag_end)?;
        Ok(())
    }
}
impl XmlWritePrimitive for u64 {
    fn write(&self, tag: &str, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error> {
        let string = self.to_string();
        string.write(tag, writer)
    }
}
impl XmlWritePrimitive for Uuid {
    fn write(&self, tag: &str, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error> {
        let string = self.to_string();
        string.write(tag, writer)
    }
}
impl<T> XmlWritePrimitive for Option<T>
where
    T: XmlWritePrimitive,
{
    fn write(&self, tag: &str, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error> {
        if let Some(value) = self.as_ref() {
            value.write(tag, writer)?;
        }
        Ok(())
    }
}

/// A trait for self-tagged objects that can be written to an XML document
pub trait XmlWrite {
    /// Writes `self` as XML element to the writer
    fn write(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error>;
}

//! Some XML helpers

use feedme_shared::{Error, Uuid};
use std::io::Write;
use xml::writer::XmlEvent;
use xml::EventWriter;

/// A trait for untagged primitives that can be written to an XML document
pub trait XmlWritePrimitive<T>
where
    T: Write,
{
    /// Writes `self` as XML element to the writer
    fn write(&self, tag: &str, writer: &mut EventWriter<T>) -> Result<(), Error>;
}
impl<T> XmlWritePrimitive<T> for String
where
    T: Write,
{
    fn write(&self, tag: &str, writer: &mut EventWriter<T>) -> Result<(), Error> {
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
impl<T> XmlWritePrimitive<T> for u64
where
    T: Write,
{
    fn write(&self, tag: &str, writer: &mut EventWriter<T>) -> Result<(), Error> {
        let string = self.to_string();
        string.write(tag, writer)
    }
}
impl<T> XmlWritePrimitive<T> for Uuid
where
    T: Write,
{
    fn write(&self, tag: &str, writer: &mut EventWriter<T>) -> Result<(), Error> {
        let string = self.to_string();
        string.write(tag, writer)
    }
}
impl<T, W> XmlWritePrimitive<W> for Option<T>
where
    T: XmlWritePrimitive<W>,
    W: Write,
{
    fn write(&self, tag: &str, writer: &mut EventWriter<W>) -> Result<(), Error> {
        if let Some(value) = self.as_ref() {
            value.write(tag, writer)?;
        }
        Ok(())
    }
}

/// A trait for self-tagged objects that can be written to an XML document
pub trait XmlWrite<T> {
    /// Writes `self` as XML element to the writer
    fn write(&self, writer: &mut EventWriter<T>) -> Result<(), Error>;
}
impl<T, W> XmlWrite<W> for Option<T>
where
    T: XmlWrite<W>,
    W: Write,
{
    fn write(&self, writer: &mut EventWriter<W>) -> Result<(), Error> {
        if let Some(value) = self.as_ref() {
            value.write(writer)?;
        }
        Ok(())
    }
}

//! Common RSS podcast feed XML tags

use crate::rss::xml_utils::{XmlWrite, XmlWritePrimitive};
use feedme_shared::{error, Error, Uuid};
use std::io::Write;
use time::{format_description::well_known::Rfc2822, OffsetDateTime};
use xml::{writer::XmlEvent, EventWriter};

/// An enclosure object
#[derive(Debug, Clone)]
pub struct Enclosure {
    /// The file size in bytes (`length`)
    pub length: u64,
    /// The file's MIME type (`type`)
    pub type_: String,
    /// The file URL (`url`)
    pub url: String,
}
impl<T> XmlWrite<T> for Enclosure
where
    T: Write,
{
    fn write(&self, writer: &mut EventWriter<T>) -> Result<(), Error> {
        // Serialize the tag
        let length = self.length.to_string();
        let tag_start = XmlEvent::start_element("enclosure")
            .attr("length", &length)
            .attr("type", &self.type_)
            .attr("url", &self.url);
        writer.write(tag_start)?;

        // Close element
        let tag_end = XmlEvent::end_element().name("enclosure");
        writer.write(tag_end)?;
        Ok(())
    }
}

/// A playlist item
#[derive(Debug, Clone)]
pub struct Item {
    /// An item's title (`title`)
    pub title: String,
    /// The item's description (`description`)
    pub description: Option<String>,
    /// An item's enclosure tag (`enclosure`)
    pub enclosure: Enclosure,
    /// An item's globally unique ID ('guid')
    pub guid: Uuid,
    /// An item's publication date (`pubDate`)
    pub pub_date: u64,
    /// An item's duration in seconds (`itunes:duration`)
    pub itunes_duration: u64,
}
impl<T> XmlWrite<T> for Item
where
    T: Write,
{
    fn write(&self, writer: &mut EventWriter<T>) -> Result<(), Error> {
        // Format the date
        let pub_date = {
            let timestamp = i64::try_from(self.pub_date).map_err(|e| error!(with: e, "timestamp is too large"))?;
            let date = OffsetDateTime::from_unix_timestamp(timestamp)?;
            date.format(&Rfc2822)?
        };

        // Write object
        writer.write(XmlEvent::start_element("item"))?;
        self.title.write("title", writer)?;
        self.description.write("description", writer)?;
        self.enclosure.write(writer)?;
        self.guid.write("guid", writer)?;
        pub_date.write("pubDate", writer)?;
        self.itunes_duration.write("itunes:duration", writer)?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

/// A channel object
#[derive(Debug, Clone)]
pub struct Channel {
    /// The playlist title (`title`)
    pub title: String,
    /// The link to the playlist website (`link`)
    pub link: Option<String>,
    /// The playlist author (`itunes:author`)
    pub itunes_author: Option<String>,
    /// The playlist description (`description`)
    pub description: Option<String>,
    /// The link to the playlist thumbnail (`itunes:image`)
    pub itunes_image: Option<String>,
    /// The playlist member items
    pub items: Vec<Item>,
}
impl<T> XmlWrite<T> for Channel
where
    T: Write,
{
    fn write(&self, writer: &mut EventWriter<T>) -> Result<(), Error> {
        // Write object
        writer.write(XmlEvent::start_element("channel"))?;
        self.title.write("title", writer)?;
        self.link.write("link", writer)?;
        self.itunes_author.write("itunes:author", writer)?;
        self.description.write("description", writer)?;
        self.itunes_image.write("itunes:image", writer)?;

        // Write items
        for item in &self.items {
            item.write(writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

/// A feed object
#[derive(Debug, Clone)]
pub struct Feed {
    /// The channel data (`channel`)
    pub channel: Channel,
}
impl<T> XmlWrite<T> for Feed
where
    T: Write,
{
    fn write(&self, writer: &mut EventWriter<T>) -> Result<(), Error> {
        // Serialize the tag
        let tag = XmlEvent::start_element("rss")
            .attr("version", "2.0")
            .ns("itunes", "http://www.itunes.com/dtds/podcast-1.0.dtd")
            .ns("content", "http://purl.org/rss/1.0/modules/content/");
        writer.write(tag)?;

        // Write the channel
        self.channel.write(writer)?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

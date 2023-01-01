//! Common RSS podcast feed XML tags

use crate::{
    error,
    error::Error,
    rss::xml_utils::{XmlWrite, XmlWritePrimitive},
    Uuid,
};
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
impl XmlWrite for Enclosure {
    fn write(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error> {
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
impl XmlWrite for Item {
    fn write(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error> {
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
impl XmlWrite for Channel {
    fn write(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error> {
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
impl XmlWrite for Feed {
    fn write(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error> {
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

/*
/// A common RSS tag
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Tag {
    /// The RSS tag (`rss`)
    Rss(Box<Tag>),
    /// The channel tag (`channel`)
    Channel(Vec<Tag>),
    /// The playlist title (`title`)
    Title(String),
    /// The playlist description (`description`)
    Description(String),
    ItunesAuthor(String),
    /// The link to the show (`link`)
    Link(String),
    /// The link to the show thumbnail (`itunes:image`)
    ItunesImage(String),
    /// An item (`item`)
    Item(Vec<Tag>),
    /// An item's title (`itunes:title`)
    ItunesTitle(String),
    /// An item's enclosure tag (`enclosure`)
    Enclosure {
        /// The file size in bytes (`length`)
        length: String,
        /// The file's MIME type (`type`)
        type_: String,
        /// The file URL (`url`)
        url: String,
    },
    /// An item's globally unique ID
    Guid(String),
    /// An item's duration in seconds (`itunes:duration`)
    ItunesDuration(String),
    /// An item's publication date (`pubDate`)
    PubDate(String),
}
impl Tag {
    /// Serializes `self`
    pub fn serialize(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error> {
        match self {
            Tag::Rss(child) => Self::rss(child, writer),
            Tag::Channel(childs) => Self::nested("channel", childs, writer),
            Tag::Title(title) => Self::value("title", title, writer),
            Tag::Description(description) => Self::value("description", description, writer),
            Tag::ItunesAuthor(itunes_author) => Self::value("itunes:author", itunes_author, writer),
            Tag::Link(link) => Self::value("link", link, writer),
            Tag::ItunesImage(itunes_image) => Self::value("itunes:image", itunes_image, writer),
            Tag::Item(childs) => Self::nested("item", childs, writer),
            Tag::ItunesTitle(itunes_title) => Self::value("itunes:title", itunes_title, writer),
            Tag::Enclosure { length, type_, url } => Self::enclosure(length, type_, url, writer),
            Tag::Guid(guid) => Self::value("guid", guid, writer),
            Tag::ItunesDuration(itunes_duration) => Self::value("itunes:duration", itunes_duration, writer),
            Tag::PubDate(pub_date) => Self::value("pubDate", pub_date, writer),
        }
    }

    /// Serializes `self`
    fn rss(child: &Tag, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error> {
        // Serialize the tag
        let tag = XmlEvent::start_element("rss")
            .attr("version", "2.0")
            .ns("itunes", "http://www.itunes.com/dtds/podcast-1.0.dtd")
            .ns("content", "http://purl.org/rss/1.0/modules/content/");
        writer.write(tag)?;

        // Serialize child
        child.serialize(writer)?;

        // Close element
        let tag_end = XmlEvent::end_element().name("rss");
        writer.write(tag_end)?;
        Ok(())
    }
    /// Serializes `self`
    fn enclosure(len: &str, type_: &str, url: &str, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error> {
        // Serialize the tag
        let tag_start = XmlEvent::start_element("enclosure").attr("length", len).attr("type", type_).attr("url", url);
        writer.write(tag_start)?;

        // Close element
        let tag_end = XmlEvent::end_element().name("enclosure");
        writer.write(tag_end)?;
        Ok(())
    }
    /// Serializes a nested generic element
    fn nested(tag: &str, childs: &[Tag], writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error> {
        // Serialize the tag
        let tag_start = XmlEvent::start_element(tag);
        writer.write(tag_start)?;

        // Serialize childs
        for child in childs {
            child.serialize(writer)?;
        }

        // Close element
        let tag_end = XmlEvent::end_element().name(tag);
        writer.write(tag_end)?;
        Ok(())
    }
    /// Serializes a generic element
    fn value(tag: &str, value: &str, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), Error> {
        // Serialize the tag
        let tag_start = XmlEvent::start_element(tag);
        writer.write(tag_start)?;

        // Write the value
        let value = XmlEvent::characters(value);
        writer.write(value)?;

        // Close element
        let tag_end = XmlEvent::end_element().name(tag);
        writer.write(tag_end)?;
        Ok(())
    }
}
*/

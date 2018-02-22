// (c) 2018 Joost Yervante Damad <joost@damad.be>

//! A quick wrapper around the [atom_syndication](https://crates.io/crates/atom_syndication) crate

#![warn(missing_docs)]

extern crate atom_syndication;
extern crate chrono;
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;


use std::fs::File;
use std::io::Write;

pub use error::Error;

use chrono::{DateTime, Local};

/// an Atom Feed
#[derive(Debug, Default)]
pub struct Feed {
    id:Option<String>,
    title:Option<String>,
    home_url:Option<String>,
    feed_url:Option<String>,
    author:Option<String>,
    email:Option<String>,
    date_updated:Option<DateTime<Local>>,
}

/// a builder for `Feed`
#[derive(Debug, Default)]
pub struct FeedBuilder(Feed);

impl FeedBuilder {

    /// set the id of the feed
    pub fn id<T:Into<String>>(mut self, id:T) -> Self {
        self.0.id = Some(id.into());
        self
    }
    
    /// set the title of the feed
    pub fn title<T:Into<String>>(mut self, title:T) -> Self {
        self.0.title = Some(title.into());
        self
    }
    
    /// set the home URL of the feed
    pub fn home_url<T:Into<String>>(mut self, home_url:T) -> Self {
        self.0.home_url = Some(home_url.into());
        self
    }
    
    /// set the feed URL of the feed
    pub fn feed_url<T:Into<String>>(mut self, feed_url:T) -> Self {
        self.0.feed_url = Some(feed_url.into());
        self
    }
    
    /// set the author of the feed
    pub fn author<T:Into<String>>(mut self, author:T) -> Self {
        self.0.author = Some(author.into());
        self
    }
    
    /// set the email address of the feed
    pub fn email<T:Into<String>>(mut self, email:T) -> Self {
        self.0.email = Some(email.into());
        self
    }
    
    /// set the updated date of the feed
    pub fn date_updated(mut self, date:DateTime<Local>) -> Self {
        self.0.date_updated = Some(date);
        self
    }

    /// build the `FeedBuilder` in a `Feed`
    pub fn build(self) -> Result<Feed, Error> {
        if self.0.id.is_none() {
            return Err("Feed id is mandatory".into())
        }
        if self.0.title.is_none() {
            return Err("Feed title is mandatory".into())
        }
        if self.0.home_url.is_none() {
            return Err("Feed home URL is mandatory".into())
        }
        if self.0.feed_url.is_none() {
            return Err("Feed URL is mandatory".into())
        }
        if self.0.author.is_none() {
            return Err("Feed author is mandatory".into())
        }
        if self.0.email.is_none() {
            return Err("Feed email is mandatory".into())
        }
        Ok(self.0)
    }
}

/// a feed `Entry`
#[derive(Debug, Default)]
pub struct Entry {
    title:Option<String>,
    url:Option<String>,
    content:Option<String>,
    date:Option<DateTime<Local>>,
    date_updated:Option<DateTime<Local>>,
    author:Option<String>,
    email:Option<String>,
}

/// a builder for `Feed`
#[derive(Debug, Default)]
pub struct EntryBuilder(Entry);

impl EntryBuilder {
    /// set the title of the entry
    pub fn title<T:Into<String>>(mut self, title:T) -> Self {
        self.0.title = Some(title.into());
        self
    }
    
    /// set the url of the entry
    pub fn url<T:Into<String>>(mut self, url:T) -> Self {
        self.0.url = Some(url.into());
        self
    }
    
    /// set the content of the entry
    pub fn content<T:Into<String>>(mut self, content:T) -> Self {
        self.0.content = Some(content.into());
        self
    }
    
    /// set the date of the entry
    pub fn date(mut self, date:DateTime<Local>) -> Self {
        self.0.date = Some(date);
        self
    }
    
    /// set the author of the entry
    pub fn author<T:Into<String>>(mut self, author:T) -> Self {
        self.0.author = Some(author.into());
        self
    }
    
    /// set the email of the entry
    pub fn email<T:Into<String>>(mut self, email:T) -> Self {
        self.0.email = Some(email.into());
        self
    }
    
    /// set the date updated of the entry
    pub fn date_updated(mut self, date_updated:DateTime<Local>) -> Self {
        self.0.date_updated = Some(date_updated);
        self
    }
    
    /// build the `EntryBuilder` in a `Entry`
    pub fn build(self) -> Result<Entry, Error> {
        if self.0.title.is_none() {
            return Err("Entry title is mandatory".into())
        }
        if self.0.url.is_none() {
            return Err("Entry url is mandatory".into())
        }
        if self.0.content.is_none() {
            return Err("Entry content is mandatory".into())
        }
        if self.0.date.is_none() {
            return Err("Entry date is mandatory".into())
        }
        Ok(self.0)
    }
}

fn make_atom_feed(feed:Feed,entries:Vec<atom_syndication::Entry>) -> Result<atom_syndication::Feed, Error> {
    let mut link = atom_syndication::Link::default();
    link.set_href(feed.home_url.unwrap());
    let mut link2 = atom_syndication::Link::default();
    link2.set_href(feed.feed_url.unwrap());
    link2.set_rel("self");
    let date_updated = feed.date_updated.unwrap_or(Local::now()).to_rfc3339();
    let feed = atom_syndication::FeedBuilder::default()
        .id(feed.id.unwrap())
        .title(feed.title.unwrap())
        .links(vec![link2, link])
        .entries(entries)
        .updated(date_updated)
        .build()?;
    Ok(feed)
}

fn make_atom_entry(feed:&Feed, entry:Entry) -> Result<atom_syndication::Entry, Error> {
    let published_date = entry.date.unwrap().to_rfc3339();
    let updated_date = entry.date_updated.map(|x| x.to_rfc3339()).unwrap_or(published_date.clone());
    
    let mut content = atom_syndication::Content::default();
    content.set_content_type("xhtml".to_string());
    content.set_value(entry.content);
    
    let person = atom_syndication::PersonBuilder::default()
        .name(entry.author.unwrap_or(feed.author.clone().unwrap()))
        .email(entry.email.unwrap_or(feed.email.clone().unwrap()))
        .uri(feed.home_url.clone())
        .build()?;
    
    let entry = atom_syndication::EntryBuilder::default()
        .title(entry.title.unwrap())
        .id(entry.url.unwrap())
        .published(published_date)
        .updated(updated_date)
        .authors(vec![person])
        .content(content)
        .build()?;
    Ok(entry)
}

/// write a feed and its entries to a writer
pub fn write_atom_feed<W:Write>(feed:Feed, entries:Vec<Entry>, output:W) -> Result<(), Error> {
    let mut v = vec![];
    for e in entries {
        let e2 = make_atom_entry(&feed, e)?;
        v.push(e2)
    }
    let feed = make_atom_feed(feed, v)?;
    feed.write_to(output)?;
    let mut s:Vec<u8> = vec![];
    feed.write_to(&mut s)?;
    let s = String::from_utf8_lossy(&s);
    debug!("feed xml: {}", s);
    Ok(())
}

/// write a feed and its entries to a file
pub fn write_atom_file(feed:Feed, entries:Vec<Entry>, filename:&str) -> Result<(), Error> {
    let out = File::create(filename)?;
    write_atom_feed(feed, entries, out)
    
}

mod error;

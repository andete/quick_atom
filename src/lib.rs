// (c) 2018 Joost Yervante Damad <joost@damad.be>

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

#[derive(Debug, Default)]
pub struct Feed {
    id:String,
    title:String,
    home_url:String,
    feed_url:String,
    author:String,
    email:String,
    date_updated:Option<DateTime<Local>>,
}

#[derive(Debug, Default)]
pub struct FeedBuilder(Feed);

impl FeedBuilder {
    
    pub fn id<T:Into<String>>(mut self, id:T) -> Self {
        self.0.id = id.into();
        self
    }
    
    pub fn title<T:Into<String>>(mut self, title:T) -> Self {
        self.0.title = title.into();
        self
    }
    
    pub fn home_url<T:Into<String>>(mut self, home_url:T) -> Self {
        self.0.home_url = home_url.into();
        self
    }
    
    pub fn feed_url<T:Into<String>>(mut self, feed_url:T) -> Self {
        self.0.feed_url = feed_url.into();
        self
    }
    
    pub fn author<T:Into<String>>(mut self, author:T) -> Self {
        self.0.author = author.into();
        self
    }
    
    pub fn email<T:Into<String>>(mut self, email:T) -> Self {
        self.0.email = email.into();
        self
    }
    
    pub fn date_updated(mut self, date:DateTime<Local>) -> Self {
        self.0.date_updated = Some(date);
        self
    }

    pub fn build(self) -> Feed {
        self.0
    }
}

#[derive(Debug)]
pub struct Entry {
    pub title:String,
    pub url:String,
    pub content:String,
    pub date:DateTime<Local>,
    pub date_updated:Option<DateTime<Local>>,
    pub author:Option<String>,
    pub email:Option<String>,
}

fn make_atom_feed(feed:Feed,entries:Vec<atom_syndication::Entry>) -> Result<atom_syndication::Feed, Error> {
    let mut link = atom_syndication::Link::default();
    link.set_href(feed.home_url);
    let mut link2 = atom_syndication::Link::default();
    link2.set_href(feed.feed_url);
    link2.set_rel("self");
    let date_updated = feed.date_updated.unwrap_or(Local::now()).to_rfc3339();
    let feed = atom_syndication::FeedBuilder::default()
        .id(feed.id)
        .title(feed.title)
        .links(vec![link2, link])
        .entries(entries)
        .updated(date_updated)
        .build()?;
    Ok(feed)
}

fn make_atom_entry(feed:&Feed, entry:Entry) -> Result<atom_syndication::Entry, Error> {
    let published_date = entry.date.to_rfc3339();
    let updated_date = entry.date_updated.map(|x| x.to_rfc3339()).unwrap_or(published_date.clone());
    
    let mut content = atom_syndication::Content::default();
    content.set_content_type("xhtml".to_string());
    content.set_value(entry.content);
    
    let person = atom_syndication::PersonBuilder::default()
        .name(entry.author.unwrap_or(feed.author.clone()))
        .email(entry.email.unwrap_or(feed.email.clone()))
        .uri(feed.home_url.clone())
        .build()?;
    
    let entry = atom_syndication::EntryBuilder::default()
        .title(entry.title)
        .id(entry.url)
        .published(published_date)
        .updated(updated_date)
        .authors(vec![person])
        .content(content)
        .build()?;
    Ok(entry)
}

pub fn make_atom<W:Write>(feed:Feed, entries:Vec<Entry>, output:W) -> Result<(), Error> {
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

pub fn make_atom_file(feed:Feed, entries:Vec<Entry>, filename:&str) -> Result<(), Error> {
    let out = File::create(filename)?;
    make_atom(feed, entries, out)
    
}

mod error;

pub mod core;
pub mod extended;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Result;

pub trait Serde
where
    Self: Serialize + DeserializeOwned,
{
    fn to_json(&self) -> Result<String> {
        let serialized = serde_json::to_string(&self);
        println!("serialized = {:?}", serialized);
        serialized
    }

    fn to_json_pretty(&self) -> Result<String> {
        let serialized = serde_json::to_string_pretty(&self);
        println!("serialized = {:?}", serialized);
        serialized
    }

    fn from_json(json: String) -> Result<Self> {
        serde_json::from_str(json.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDate, Utc};
    use pretty_assertions::assert_eq;

    use crate::{core::*, extended::*};

    // A set of tests from https://www.w3.org/TR/activitystreams-vocabulary examples
    #[test]
    fn example_1() {
        let listing = r#"{
          "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
          "type": "Object",
          "id": "http://www.test.example/object/1",
          "name": "A Simple, non-specific object"
        }"#;
        let object: Object<Null> = Document::from_json(String::from(listing)).unwrap().object;
        assert_eq!(object.object_type, Some(String::from("Object")));
        assert_eq!(
            object.id,
            Some(String::from("http://www.test.example/object/1"))
        );
        assert_eq!(
            object.name,
            Some(String::from("A Simple, non-specific object"))
        );
    }

    #[test]
    fn example_2() {
        let listing = r#"
      {
        "@context": {"@vocab": "https://www.w3.org/ns/activitystreams"},
        "type": "Link",
        "href": "http://example.org/abc",
        "hreflang": "en",
        "mediaType": "text/html",
        "name": "An example link"
      }
      "#;

        let link: Link = Document::from_json(String::from(listing)).unwrap().object;
        assert_eq!(link.link_type, "Link");
        assert_eq!(link.href.href, "http://example.org/abc");
        assert_eq!(link.hreflang, Some(String::from("en")));
        assert_eq!(link.href.media_type, Some(String::from("text/html")));
        assert_eq!(link.name, Some(String::from("An example link")));
    }

    #[test]
    fn example_3() {
        let listing = r#"
      {
        "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
        "type": "Activity",
        "summary": "Sally did something to a note",
        "actor": {
          "type": "Person",
          "name": "Sally"
        },
        "object": {
          "type": "Note",
          "name": "A Note"
        }
      } 
      "#;

        let activity: Activity = Document::from_json(String::from(listing)).unwrap().object;
        assert_eq!(activity.object_type, Some(String::from("Activity")));
        assert_eq!(
            activity.summary,
            Some(String::from("Sally did something to a note"))
        );

        assert!(activity.actor.is_some());
        let actor = activity.actor.unwrap();
        assert_eq!(actor.object_type, Some(String::from("Person")));
        assert_eq!(actor.name, Some(String::from("Sally")));

        assert!(activity.object.is_some());
        let object = activity.object.unwrap();
        assert_eq!(object.object_type, Some(String::from("Note")));
        assert_eq!(object.name, Some(String::from("A Note")));
    }

    #[test]
    fn example_4() {
        let listing = r#"
      {
        "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
        "type": "Travel",
        "summary": "Sally went to work",
        "actor": {
          "type": "Person",
          "name": "Sally"
        },
        "target": {
          "type": "Place",
          "name": "Work"
        }
      }
      "#;

        let activity: IntransitiveActivity =
            Document::from_json(String::from(listing)).unwrap().object;
        assert_eq!(activity.object_type, Some(String::from("Travel")));
        assert_eq!(activity.summary, Some(String::from("Sally went to work")));

        assert!(activity.actor.is_some());
        let actor = activity.actor.as_ref().unwrap();
        assert_eq!(actor.object_type, Some(String::from("Person")));
        assert_eq!(actor.name, Some(String::from("Sally")));

        assert!(activity.target.is_some());
        let target = activity.target.as_ref().unwrap();
        assert_eq!(target.object_type, Some(String::from("Place")));
        assert_eq!(target.name, Some(String::from("Work")));
    }

    #[test]
    fn example_5() {
        let listing = r#"
        {
          "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
          "summary": "Sally's notes",
          "type": "Collection",
          "totalItems": 2,
          "items": [
            {
              "type": "Note",
              "name": "A Simple Note"
            },
            {
              "type": "Note",
              "name": "Another Simple Note"
            }
          ]
        }
      "#;

        let collection: Collection<Object<Null>> =
            Document::from_json(String::from(listing)).unwrap().object;
        assert_eq!(collection.object_type, Some(String::from("Collection")));
        assert_eq!(collection.summary, Some(String::from("Sally's notes")));
        assert_eq!(collection.total_items, Some(2));

        let items = &collection.items;
        assert_eq!(items.len(), collection.total_items.unwrap());
        assert_eq!(items[0].object_type, Some(String::from("Note")));
        assert_eq!(items[0].name, Some(String::from("A Simple Note")));
        assert_eq!(items[1].object_type, Some(String::from("Note")));
        assert_eq!(items[1].name, Some(String::from("Another Simple Note")));
    }

    #[test]
    fn example_6() {
        let listing = r#"
      {
        "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
        "summary": "Sally's notes",
        "type": "OrderedCollection",
        "totalItems": 2,
        "orderedItems": [
          {
            "type": "Note",
            "name": "A Simple Note"
          },
          {
            "type": "Note",
            "name": "Another Simple Note"
          }
        ]
      }
      "#;
        let collection: OrderedCollection<Object<Null>> =
            Document::from_json(String::from(listing)).unwrap().object;
        assert_eq!(
            collection.object_type,
            Some(String::from("OrderedCollection"))
        );
        assert_eq!(collection.summary, Some(String::from("Sally's notes")));
        assert_eq!(collection.total_items, Some(2));

        let items = &collection.ordered_items;
        assert_eq!(items.len(), collection.total_items.unwrap());
        assert_eq!(items[0].object_type, Some(String::from("Note")));
        assert_eq!(items[0].name, Some(String::from("A Simple Note")));
        assert_eq!(items[1].object_type, Some(String::from("Note")));
        assert_eq!(items[1].name, Some(String::from("Another Simple Note")));
    }

    #[test]
    fn example_7() {
        let listing = r#"
      {
        "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
        "summary": "Page 1 of Sally's notes",
        "type": "CollectionPage",
        "id": "http://example.org/foo?page=1",
        "partOf": "http://example.org/foo",
        "items": [
          {
            "type": "Note",
            "name": "A Simple Note"
          },
          {
            "type": "Note",
            "name": "Another Simple Note"
          }
        ]
      }
      "#;
        let collection_page: CollectionPage<Object<Null>> =
            Document::from_json(String::from(listing)).unwrap().object;
        assert_eq!(
            collection_page.object_type,
            Some(String::from("CollectionPage"))
        );
        assert_eq!(
            collection_page.id,
            Some("http://example.org/foo?page=1".to_string())
        );
        assert_eq!(
            collection_page.summary,
            Some(String::from("Page 1 of Sally's notes"))
        );
        assert_eq!(
            collection_page.part_of,
            "http://example.org/foo".to_string()
        );
        assert_eq!(collection_page.total_items, None);

        let items = &collection_page.items;
        assert_eq!(items[0].object_type, Some(String::from("Note")));
        assert_eq!(items[0].name, Some(String::from("A Simple Note")));
        assert_eq!(items[1].object_type, Some(String::from("Note")));
        assert_eq!(items[1].name, Some(String::from("Another Simple Note")));
    }

    #[test]
    fn example_8() {
        let listing = r#"
{
  "@context": {"@vocab": "https://www.w3.org/ns/activitystreams"},
  "summary": "Page 1 of Sally's notes",
  "type": "OrderedCollectionPage",
  "id": "http://example.org/foo?page=1",
  "partOf": "http://example.org/foo",
  "orderedItems": [
    {
      "type": "Note",
      "name": "A Simple Note"
    },
    {
      "type": "Note",
      "name": "Another Simple Note"
    }
  ]
}
"#;
        let collection_page: OrderedCollectionPage<Object<Null>> =
            Document::from_json(String::from(listing)).unwrap().object;
        assert_eq!(
            collection_page.object_type,
            Some(String::from("OrderedCollectionPage"))
        );
        assert_eq!(
            collection_page.id,
            Some("http://example.org/foo?page=1".to_string())
        );
        assert_eq!(
            collection_page.summary,
            Some(String::from("Page 1 of Sally's notes"))
        );
        assert_eq!(
            collection_page.part_of,
            "http://example.org/foo".to_string()
        );
        assert_eq!(collection_page.total_items, None);

        let items = &collection_page.ordered_items;
        assert_eq!(items[0].object_type, Some(String::from("Note")));
        assert_eq!(items[0].name, Some(String::from("A Simple Note")));
        assert_eq!(items[1].object_type, Some(String::from("Note")));
        assert_eq!(items[1].name, Some(String::from("Another Simple Note")));
    }

    #[test]
    fn example_53() {
        let listing = r#"{
          "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
          "type": "Note",
          "name": "A Word of Warning",
          "content": "Looks like it is going to rain today. Bring an umbrella!"
        }"#;
        let document: Document<Note> = Document::from_json(String::from(listing)).unwrap();
        let note = document.object;
        assert_eq!(note.object_type, Some(String::from("Note")));
        assert_eq!(note.name, Some(String::from("A Word of Warning")));
        assert_eq!(
            note.content,
            Some(String::from(
                "Looks like it is going to rain today. Bring an umbrella!"
            ))
        );
    }

    #[test]
    fn example_69() {
        let listing = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "name": "Holiday announcement",
  "type": "Note",
  "content": "Thursday will be a company-wide holiday. Enjoy your day off!",
  "audience": {
    "type": "http://example.org/Organization",
    "name": "ExampleCo LLC"
  }
}"#;
        let document: Document<Object<Null>> = Document::from_json(String::from(listing)).unwrap();
        let object = document.object;
        assert_eq!(object.name, Some(String::from("Holiday announcement")));
        assert_eq!(object.object_type, Some(String::from("Note")));
        assert_eq!(
            object.content,
            Some(String::from(
                "Thursday will be a company-wide holiday. Enjoy your day off!"
            ))
        );
        assert!(object.audience.is_some());
        let audience = object.audience.unwrap();
        assert_eq!(
            audience.object_type,
            Some(String::from("http://example.org/Organization"))
        );
        assert_eq!(audience.name, Some(String::from("ExampleCo LLC")));
    }

    #[test]
    fn example_114() {
        let listing = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "summary": "A simple note",
  "type": "Note",
  "content": "A <em>simple</em> note"
}"#;
        let document: Document<Object<Null>> = Document::from_json(String::from(listing)).unwrap();
        let object = document.object;
        assert_eq!(object.summary, Some(String::from("A simple note")));
        assert_eq!(object.object_type, Some(String::from("Note")));
        assert_eq!(object.content, Some(String::from("A <em>simple</em> note")));
    }

    #[test]
    fn example_133() {
        let listing = r#"{
        "@context": {
          "@vocab": "https://www.w3.org/ns/activitystreams"
        },
        "name": "Cane Sugar Processing",
        "type": "Note",
        "summary": "A simple <em>note</em>"
      }"#;
        let document: Document<Object<Null>> = Document::from_json(String::from(listing)).unwrap();
        let object = document.object;
        assert_eq!(object.summary, Some(String::from("A simple <em>note</em>")));
        assert_eq!(object.object_type, Some(String::from("Note")));
        assert_eq!(object.name, Some(String::from("Cane Sugar Processing")));
    }

    // A set of tests from https://www.w3.org/TR/activitystreams-core/ examples
    #[test]
    fn minimal_activity_3_1() {
        let actual = Document::new(
            ContextBuilder::new().build(),
            ActivityBuilder::new(
                String::from("Create"),
                String::from("Martin created an image"),
            )
            .actor(
                ActorBuilder::new(String::from("Person")).id("http://www.test.example/martin"
                    .parse::<http::Uri>()
                    .unwrap()),
            )
            .object(
                ObjectBuilder::new().id("http://example.org/foo.jpg".parse::<http::Uri>().unwrap()),
            )
            .build(),
        );
        let expected = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Create",
  "summary": "Martin created an image",
  "actor": {
    "type": "Person",
    "id": "http://www.test.example/martin"
  },
  "object": {
    "id": "http://example.org/foo.jpg"
  }
}"#;
        assert!(actual.to_json_pretty().is_ok());
        assert_eq!(actual.to_json_pretty().unwrap(), expected);
    }

    #[test]
    fn basic_activity_with_additional_detail_3_2() {
        let actual = Document::new(
            ContextBuilder::new().build(),
            ActivityBuilder::new(
                String::from("Add"),
                String::from("Martin added an article to his blog"),
            )
            // TODO: figure out how to get a 'Z' on this. probably requires a time-zone (so not naive)
            .published(DateTime::<Utc>::from_utc(
                NaiveDate::from_ymd(2015, 2, 10).and_hms(15, 4, 55),
                Utc,
            ))
            .actor(
                ActorBuilder::new(String::from("Person"))
                    .id("http://www.test.example/martin"
                        .parse::<http::Uri>()
                        .unwrap())
                    .name(String::from("Martin Smith"))
                    .url("http://example.org/martin".parse::<http::Uri>().unwrap())
                    .image(LinkBuilder::new(
                        UriBuilder::new(
                            "http://example.org/martin/image.jpg"
                                .parse::<http::Uri>()
                                .unwrap(),
                        )
                        .media_type(String::from("image/jpeg")),
                    )),
            )
            .object(
                ObjectBuilder::new()
                    .object_type(String::from("Article"))
                    .id("http://www.test.example/blog/abc123/xyz"
                        .parse::<http::Uri>()
                        .unwrap())
                    .name(String::from("Why I love Activity Streams"))
                    .url(
                        "http://example.org/blog/2011/02/entry"
                            .parse::<http::Uri>()
                            .unwrap(),
                    ),
            )
            .target(
                ObjectBuilder::new()
                    .object_type(String::from("OrderedCollection"))
                    .id("http://example.org/blog/".parse::<http::Uri>().unwrap())
                    .name(String::from("Martin's Blog")),
            )
            .build(),
        );
        let expected = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Add",
  "published": "2015-02-10T15:04:55Z",
  "summary": "Martin added an article to his blog",
  "actor": {
    "type": "Person",
    "id": "http://www.test.example/martin",
    "name": "Martin Smith",
    "url": "http://example.org/martin",
    "image": {
      "type": "Link",
      "href": "http://example.org/martin/image.jpg",
      "mediaType": "image/jpeg"
    }
  },
  "object": {
    "type": "Article",
    "id": "http://www.test.example/blog/abc123/xyz",
    "name": "Why I love Activity Streams",
    "url": "http://example.org/blog/2011/02/entry"
  },
  "target": {
    "type": "OrderedCollection",
    "id": "http://example.org/blog/",
    "name": "Martin's Blog"
  }
}"#;
        assert!(actual.to_json_pretty().is_ok());
        assert_eq!(actual.to_json_pretty().unwrap(), expected);
    }

    #[test]
    fn object_4_1_7() {
        let actual = Document::new(
            ContextBuilder::new().build(),
            ObjectBuilder::new()
                .id("http://example.org/foo".parse::<http::Uri>().unwrap())
                .object_type(String::from("Note"))
                .name(String::from("My favourite stew recipe"))
                .published(DateTime::<Utc>::from_utc(
                    NaiveDate::from_ymd(2014, 8, 21).and_hms(12, 34, 56),
                    Utc,
                ))
                .add_attributed_to(
                    ActorBuilder::new(String::from("Person"))
                        .id("http://joe.website.example/".parse::<http::Uri>().unwrap())
                        .name(String::from("Joe Smith"))
                        .build(),
                )
                .build(),
        );

        let expected = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Note",
  "id": "http://example.org/foo",
  "name": "My favourite stew recipe",
  "published": "2014-08-21T12:34:56Z",
  "attributedTo": [
    {
      "type": "Person",
      "id": "http://joe.website.example/",
      "name": "Joe Smith"
    }
  ]
}"#;
        assert!(actual.to_json_pretty().is_ok());
        assert_eq!(actual.to_json_pretty().unwrap(), expected);
    }
}

window.SIDEBAR_ITEMS = {"struct":[["Activity","An [Activity] is a subtype of [Object] that describes some form of action that may happen, is currently happening, or has already happened. The [Activity] type itself serves as an abstract base type for all types of activities. It is important to note that the [Activity] type itself does not carry any specific semantics about the kind of action being taken."],["ActivityBuilder","Builder for an [Activity]."],["Collection","A [Collection] is a subtype of [Object] that represents ordered or unordered sets of [Object] or [Link] instances. Refer to the Activity Streams 2.0 Core specification for a complete description of the [Collection] type."],["CollectionBuilder","Builder for a [Collection]."],["CollectionPage","Used to represent distinct subsets of items from a [Collection]. Refer to the Activity Streams 2.0 Core for a complete description of the [CollectionPage] object."],["CollectionPageBuilder","Builder for a [CollectionPage]."],["Context","JSON-LD uses the special @context property to define the processing context. The value of the @context property is defined by the [JSON-LD] specification. Implementations producing Activity Streams 2.0 documents should include a @context property with a value that includes a reference to the normative Activity Streams 2.0 JSON-LD @context definition using the URL “https://www.w3.org/ns/activitystreams”. Implementations may use the alternative URL “http://www.w3.org/ns/activitystreams” instead. This can be done using a string, object, or array. https://www.w3.org/TR/activitystreams-core/#jsonld"],["ContextBuilder","Builder struct for [Context]."],["Document","Outer object for serialization and deserialization. Not an Activity Streams 2.0 object."],["IntransitiveActivity","Instances of [IntransitiveActivity] are a subtype of [Activity] representing intransitive actions. The object property is therefore inappropriate for these activities."],["IntransitiveActivityBuilder","Builder for an [IntransitiveActivity]."],["Link","A [Link] is an indirect, qualified reference to a resource identified by a URL. The fundamental model for links is established by RFC5988. Many of the properties defined by the Activity Vocabulary allow values that are either instances of [Object] or [Link]. When a [Link] is used, it establishes a qualified relation connecting the subject (the containing object) to the resource identified by the href. Properties of the [Link] are properties of the reference as opposed to properties of the resource."],["LinkBuilder","Builder for a [Link] struct."],["Null","[Null]-type object that implements [Serde] for convenience"],["Object","The [Object] is the primary base type for the Activity Streams vocabulary. In addition to having a global identifier (expressed as an absolute IRI using the id property) and an “object type” (expressed using the type property), all instances of the Object type share a common set of properties normatively defined by the Activity Vocabulary. These include: attachment | attributedTo | audience | content | context | contentMap | name | nameMap | endTime | generator | icon | image | inReplyTo | location | preview | published | replies | startTime | summary | summaryMap | tag | updated | url | to | bto | cc | bcc | mediaType | duration All properties are optional (including the id and type)."],["ObjectBuilder","Builder for [Object]."],["OrderedCollection","A subtype of [Collection] in which members of the logical collection are assumed to always be strictly ordered."],["OrderedCollectionBuilder","Builder for an [OrderedCollection]."],["Preview","Identifies an entity that provides a preview of this object."],["PreviewBuilder","Builder for [Preview]."],["Uri","A utility struct to describe a URI."],["UriBuilder","Builder struct for [Uri]."]]};
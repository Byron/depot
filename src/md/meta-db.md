## Meta Database Brainstorm

### goals

* provide an API for using a data model. The latter is powerful enough to satisfy most storage needs, and can natively handle connections between things
* the data model can be represented in many programming languages
* maps data from any data source into its data model
* allows links between items from different data sources
* supports transactions when changing data within a single db at least ( its technically impossible for transactions to span multiple dbs )
* powerful event system allows various plugins to respond to change, which can in turn push it out over sockets.
* it is possible to connect to another instance of a similarly configured meta-database, and seamlessly access all the internal database it provides (proxying).
* each stored value is identifiable by some sort of URL. These can be resolved to quickly find the respective item.
* even though transactions should be part of the system, in-memory-only applications should have no trouble with it or much overhead.
* constraints allow to specify that, for example, upon deletion of one entity, or disconnection of a particular property, the respective entity should be deleted automatically as well. This functionality is similar to what's available in SQL, which greatly helps to keep a model conistent automatically.
    * Alternatively, or as a first step, one could consider implementing some sort of garbage collector, which determines which entities are stray and can be deleted for that reason.

### data model

* nodes have properties, recursively. Properties can be connected. A connection is special kind of node, which may have properties (?)

* historical values (in separate DB). Learn from the past. Easy to make projects, estimates based on some algorithm.
* streams should be supported, as such they can become available, to allow processors to continuously read the stream, and possibly write the adjusted result. How do streams work in a transactional system, after all streams can't work within such bounds.

### architecture

* in process lock bases multithreading is nogo
* offloading io to worker(s) possible, but working with async calls might be cumbersome, api might be cluttered
* simple gui thread,background thread model seems like minimum fit, gui should always be responsive 
* changes are pushed to registered clients automatically, allowing them to update their view accordingly.
* zeromq seems intruiging as it would enforce a protocol that is quite language agnostic, while allowing multiple threads/processes/hosts to interact peacefully
* you can query the interface schema to pre-select attributes you are interested in querying/editing, by querying property meta-data
* copy-on-write semantics can be turned on on demand. This is useful for shared nodes, which should be unshared when they change in order to keep the entity using it stable. (e.g. a project is finished, and you want to be sure it stays as is even if shared entities are changing (like output formats)). The latter could also be done by having multiple versions of something around, and just freeze the version.
* multi-dimensional values, such as time-varying, locations, or locations in time.
  + this is much like having a context, that value providers can use to adjust their computation, and which is shared among all value providers which are asked to compute.
  + Overrides to the context should be possible, on global level, or per interface.
* Allow offline writes and schedule them according to their interdependencies
    * consider complex creation and connection of items, creating dependencies that have to be adhered to when making the changes in an object database. For instance, it's not possible to create a connection to a object that doesn't yet exist, and it's not usually feasible to create each object individually as it provokes roundtrips. On the other hand, when keeping track in order or occurrence, things should be fine if grouped be operation as well (like create object, change value/connection)
* **A vital requirement** is the ability to make connections, but also to allow 'push' semantics to be added on top and after the fact. That way, value changes can propagate automatically like signal/slot mechanisms.
    - solving this could either be done using transactions, maybe with delegates being in the mix. Possibly it's viable to add this functionality to per-session delegates, which get to process all value changes. For instance, pub/sub should be implemented that way too.
* anything is identifieable through a URL, e.g. entities instances as well as properties on particular instances. Entity Session Managers (those who make a root entity available) can be access through urls as well. This must play nicely with URLs of existing REST based web applications, as these must be usable verbatim.

### Questions

* Could protocol buffers as general purpose definition language be suitable to make the schema cross-platform ? After all, we want to send data around all the time, in-process, inter-process, and across machines at some point.
* What about web-friendlyness ? RPC via json/http friendly protocols might be something nice to have, or at least should be relatively easy to implement.
* Can can [camlistore](http://camlistore.org) be useful ? It's written in go, and could make a nice database backend. In the end, it would be just a backend.
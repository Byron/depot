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

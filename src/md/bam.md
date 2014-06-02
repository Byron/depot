## General

* Whatever happens in BAM, it must be easy to oursource particular tasks. This doesn't mean BAM should natively support queuing, but building in the notion of an 'evaluator' would certainly help. The idea is to quickly shoot of something that produces reviewable material without having to interrupt your work.
* As with KVStore based application settings, it must be easy to define general preferences, but allow the user to override it (and store it per user, or directly within the scene). For instance, publish settings really have to be stored with the scene to be useful, but defaults should be customizable depending on the context. Thus, unset values should be looked up in one database (i.e. kvstore), but written to another (i.e. scene local storage)
* **transactional operations:** All File output should be done to a temporary storage, to allow copying the result when the entire operation is complete. It should be possible to push changes more often (i.e. after each rendered frame), but delete the result on failure. As some outputs are expensive, and as reviews might help fixing the problem, it should be possible to keep any result, but clearly mark it as incomplete or failed to prevent further use downstream.
* Wherever code is executed, a preconfigured logger instance must be available. It can be used to record issues, and possibly even to take progress information, bundling it all up into a particularly formatted information stream.
* Platforms and their properties (like OSX, linux, ...) must not be hardcoded, but need to be retrieved from some sort of settings repository, like the kvstore.

## Name Generators

* It must be possible to heavily delay folder creation, if so desired. Usually, you want certain structure to always be present, whereas other structure should only be created when someone is about to start working with it.
* Optional fields seem to be like something real nice to have, even though they might make path inference so much harder. The implementation would most likely just have multipple formats, one with and without the optional field. The more optional fields, the more possible combinations.



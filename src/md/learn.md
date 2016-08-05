# Mixed Webtech
* **Chrome Extensions**
	* redux dev tools - needs hook in form of reducers
	* react dev tools
* **Programming Languages**
	* LiveScript
		* CoffeeScript on steroids !
	* ES6
		* *spread operator*: `[...some_list, 1, 2, ...other_list]`
		* `Object.assign(base, OBJ..OBJ-N)`
* **Atom Plugins**
	* Emmet - simplified creation of xml trees
	* source-preview - live preview of jsx and coffeescript
* **Frameworks**
	* *React*
		* JSX just just optional
		* Components have a `state`, which is treated as immutable, and can
		  be set using `setState`. The latter also causes render to be invoked
		  if something did indeed change.
		* `forceUpdate()` on component also triggers re-rendering.
		* use action creators (closures returning an action) to decouple Actions
		  from their source, allowing others to use them too.
	* *Redux*
		* Action have types, and if replayed, recreate any application state
		* State which is immutable (using Immutable.js)
		* Dispatch an action to change state
		* Reducers are pure functions which transform previous state to new one,
			by returning it.
			* **remember**: reducers should take care of related concerns only.
		* use `connect` to link selected state to your `props` member. Saves you
		  from having to pass state all the time.
		* *representational component* is a component that doesn't dispatch, or set
		  state, but instead uses plain callback properties. These are most
			flexible, and easy to test as they are *pure*.
		* *container components* usually use `connect` to bind state and callbacks
		  to a respective presentational component right away.
		* *middleware* can be used to make new actions pass through custom handlers.
			This works by patching `dispatch` of the redux store, and allowing middle
			wares to call each other.
		  They can change the flow of events in every which way, and read the state.
			It's called like `middleWare(dispatch, getState)`.
		* You should try to maximize decoupling, to the point where your props
		  are the only thing that's needed. Use `connect` to facilitate binding.
		* If `connect` doesn't depend on redux state, don't provide a
		  `stateToProps()` method, but use `null`
		* `connect` only seems to work with context from the `Provider`.
	* *react-redux*
		* `Provider` component can be used to pass context to children by placing
		 a `props` like object in a sibling field called `context`.
		 Children must explicitly enable the context they want to receive in order
		 to work with it. It turns out that function-components (e.g.
		 `(props) => {}`) receive the context via a second argument, e.g.
			`(props, context) => {}`. But careful, check if this API is stable.

	* *Deep Freeze*
		* prevent application state from changing ... .
	* *Rambda*
	  * function composition and higher kinded things.

# Concourse

## Killer Features

* **unify build environments by making tasks run in
   containers**
    - these are easily controlled via the commandline
    - no more spamming your host system ... and reproducible results.
* **workers work on linux, osx, windows**
    - support for custom docker registries
* **test locally by default**
		- setup VM with a single command, and test everything before pushing.

## Concepts

*   Job
   - Spawns builds from its plan, whose results are placed in a single
     directory. It is shared across all resources and tasks build in
     order according to *plan*. Configured via task *inputs* and *outputs*.
*   Task
   - has a *plan*, whose instances are called *builds*
*   resource (*versioned*)

Versioned resources flow through jobs, yielding a graph.
Jobs can process a version of a *resource*, and can be triggered
automatically, or
manually. Jobs can fail, succeed, or error. The latter happens if a
*resource*
version could not be retrieved. If failure happens, the *resource* versions
provided as input will not be *passed* to downstream jobs.

## Runtime

*   Each resources runs in a container, which implements 3 callbacks via scripts
*   Resources persist, and thus have state
*   can receive parameters on each `get` and `put` invocation

    *   `check` - discover new versions or the first one
    *   `in` - read a version
    *   `out` - write/create a new version

*   Each *jobs* runs in a container

    *   run only if all input versions are valid
    *   *resource* versions are available in the container root, side by side
        in directories named after their *resource* name.
    *   environment variables can be set via `config.params`.
    *   *trigger*'ed manually by default, but can be set to *trigger*
        automatically if an input version changes.

*   The *main service* runs in a container, coordinating the pipelines
*   *Workers* are running *Task* containers, possibly multiple at a time.

## Structure

*   \*Pipeline -> \*Job -> \*task -> \**resource*

## Serialization

*   pipelines are defined in yaml files, connecting resources and jobs with
    each other.
*   Tasks can be specified in their own yaml files, which are versioned
    alongside their source.

## Panic Buttons

You can *disable*

*   entire pipelines
*   jobs
*   resources
*   versions

## Parallelization

* multiple *builds* of a *resource* set can run in parallel
* You can force *builds* that depend on the same possibly mutating resource to run serially, using *serial_groups* for tasks. It's like a mutex on a resource, assuring the resource will only be changed in order of flow, build by build, version by version.
* You can force all *build* to run one after another, in case they share an external (possibly unknown) resource.

## Authentication

Support for multiple auth standards, like

* **basic auth**
* **oauth2**
* **none** - used in vagrant box in `concourse/lite`

Auth tokens (received in all the cases except for no-auth) are
stored in `~/.flyrc`.

## Secrets

*   passed on the command-line (`fly`) as strings or via .yml file. They are
    contained in or provided as clear text, and substituted into their
    designated places later via simple `{{my-secret}}` syntax.

## Tricks

*   Use standalone *Jobs* with resources to bump versions, for example
*   intercept any container using `fly intercept`
*   intercept *resource* containers using `fly intercep --check pipe/resource`
*   run individual commands with `fly intercept <cmd>`
*   use `fly intercept` to see working directory and environment variables
    usable by the build.
*   Submit local directory to *task* and execute it. $PWD name will be used as
    name of input resource, and you might have to setup a mapping betwen them
    with `-i <resource-name>=.`
*   Use `fly watch` to see logs of some (or specified) *build*, similar to
    what the GUI provides.
*   Monitor *workers*, *containers* and *volumes* to get an idea of your
    infrastructure.
*   Use [checkman](https://github.com/cppforlife/checkman) in place of
    `cc-menu`.

## (Possible) Caveats

*   It's not too easy to re-run a Task (with `fly`) using specified resource
    versions as input
*   Needs docker images, pulled via any docker repository. You should have
    your own at least.

## Concurrent Access

* multiple people using certain `fly` commands at the same time might cause
  undefined results.

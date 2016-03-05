# Concourse

## Killer Features

* **unify build environments by making tasks run in
   containers**
    - these are easily controlled via the commandline
    - no more spamming your host system ... and reproducible results.
* **workers work on linux, osx, windows**
    - support for custom docker registries

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
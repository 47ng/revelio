# Revelio Project Structure

At the moment, the project is organised into a Cargo workspace, containing
fine-grained libs that each have a specific function, and inter-depend on each
other.

While it would make little sense to publish these internal libraries (as they
provide little to no value to other projects), the next stages of the project
may require a different segmentation approach.

At the moment, the only end-product is the `revelio` CLI tool, but in the future
it might include:

- Some serverless endpoints (using Now.sh)
- Stateful web services (running in Docker containers)
- The `revelio` CLI compiled for various platforms

Most of the segmentation is related to the verification function of Revelio, as
the generation is confined to the CLI and will only run in CIs.

One issue is that Now's rust builder does not really play by the Cargo rules,
and this throws off VSCode's integration with RLS. For this reason, the Now
serverless endpoints should not be included in the Cargo workspace, but then
we need to see if we can still link to local filesystem crates.
=> Yes, we can, but we can't deploy to Now with that, as the build occurs on
their servers, and so has to use published dependencies.

The whole point of a workspace is to share:

- The `target` output directory
- The Cargo.lock file

This makes most sense for a single output made up of smaller related libraries,
but as long as a crate is shared among multiple larger (or unrelated) outputs,
a workspace might not be the best choice.

We can split the project into the following parts:

- The `revelio` crate, made of:
  - Shared type and traits definitions
  - The verification system
  - The generation system (potentially)
- The Revelio CLI executable, made of:
  - CLI stuff from outside
  - The `revelio` crate
- Some web services (serverfull or serverless), leaning on top of the rest

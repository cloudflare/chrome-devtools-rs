# Release Checklist

This is a list of the things that need to happen during a release.

## Build a Release

### Prepare the Changelog (Full release only)

1. Open the associated milestone. All issues and PRs should be closed. If they
   are not you should reassign all open issues and PRs to future milestones.
1. Go through the commit history since the last release. Ensure that all PRs
   that have landed are marked with the milestone. You can use this to show all
   the PRs that are merged on or after YYY-MM-DD:
   `https://github.com/issues?utf8=%E2%9C%93&q=repo%3Acloudflare%2Fchrome-devtools-rs+merged%3A%3E%3DYYYY-MM-DD`
1. Go through the closed PRs in the milestone. Each should have a changelog
   label indicating if the change is docs, fix, feature, or maintenance. If
   there is a missing label, please add one.
1. Choose an emoji for the release. Try to make it semi-related to something
   that's been included in the release (point releases can be a little weirder).
1. Add this release to the `CHANGELOG.md`. Use the structure of previous
   entries. If you use VS Code, you can use
   [this snippet](https://gist.github.com/cloudflare/9feaf56b3dfe2a2c4ad156db36f2f75a)
   to insert new changelog sections. If it is a release candidate, no official
   changelog is needed, but testing instructions will be added later in the
   process.

### Update cargo manifest

1. Update the version in `Cargo.toml`.
1. Run `cargo update`.
1. Run `cargo test`.
1. Run `cargo build`.

### Release

1. Create a new branch "#.#.#" where "#.#.#" is this release's version (release)
   or "#.#.#-alpha.#"
1. Push up a commit with the `Cargo.toml`, `Cargo.lock`, and `CHANGELOG.md`
   changes. The commit message should be the same as your branch name from
   step 1.
1. Once ready to merge, tag the commit by running either
   `git tag -a v#.#.# -m #.#.#` (release), or
   `git tag -a v#.#.#-alpha.# -m #.#.#`.
1. Run `git push --tags`.
1. Run `cargo publish`.

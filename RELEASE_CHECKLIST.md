# Release Checklist

In order to release a new version to Hex.pm we need to:

1. List the changes in the `CHANGELOG.md` file
2. Update the `README.md`, `CHANGELOG.md` and `mix.exs` with the new version
3. Commit and create a tag for that version
4. Push the changes to the repository with: `git push origin main --tags`
5. Wait for the CI to build all release artifacts
6. Run `mix rustler_precompiled.download MailParser --all --print`
7. Copy the output of the mix task and add it to the release notes
8. Run `mix hex.publish` and **make sure the checksum file is present**
   in the list of files to be published.

It is important to make sure that we publish the checksum file along with the
package, otherwise users will not be able to use the library with precompiled
files. In this case, the compilation would always be enforced.

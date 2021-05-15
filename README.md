The main site of `image-rs` at <www.image-rs.org>.

## Contributing

Any member of the organization is welcome to contribute to the site, not only
collaborators. The main branch that is served over `www.image-rs.org` should
only be changed through PRs with at least one review, regardless of the author.
Major changes to the structure or look should gather some feedback over a week
but at least three days (so that one is a work day).

Most pressing issues:

* Design the pages instead of relying on a rather unstructured default look.
* Establish a system for tags and categories
* Evaluate how to present the individual projects under the organization. Each
  projects could have one dedicated maintainer on its project page.

## Serving locally

For basic usage, follow [the official Github guide][GHGuide]. If you are not
comfortable giving `sudo` to the `bundler` commands just for this project (i.e.
this is not your main project or you created a project related account to
confine access, ...) like me then you can also do:

1. Create some local directory used to hold the installed gems. I use
   `GEM_HOME=~/.local/lib/gems` to mirror the root tree that would otherwise be used.
2. Invoke `bundle install --path $GEM_HOME`. Note that in newer bundle versions
   you will simply want to use once `bundle config set --local path $GEM_HOME`
   instead of the `--path` argument.
3. Now Invoke jekyll through `bundle` as well
   > `bundle exec jekyll serve`

[GHGuide]: https://help.github.com/en/articles/setting-up-your-github-pages-site-locally-with-jekyll

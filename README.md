# mdiff

tiny cli utility to render a rich diff of a markdown file from git history.

any cli arguments given will be passed to `git diff` which should return a single file diff.

example: `HEAD~1..HEAD README.md`

this will then be parsed and the diff markers replaced with html `<span>` tags
the script also includes a style tag to give these spans some color

to create a full html document, use something like [pandoc](https://pandoc.org):

```sh
mdiff HEAD~1..HEAD README.md | pandoc -f markdown - -o diff.html --standalone
```

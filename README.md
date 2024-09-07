CI cli.

## Commands

`watch (alias: w)`: poll the latest CI run's status, notify on finalization.

`start (alias: s)`

`ls`

`open (alias: o)`: open the logs in browser.

## Setup

Requests default to the current git branch, `origin`'s url and use `gh` for auth.

The `-b` arg for branch and env variables:
```
REPO=
GH_TOKEN=
```
for the rest, override this.

(optional)
You can symlink [a notification sound](https://mixkit.co/free-sound-effects/notification/) as `$HOME/sound.mp3` to use `watch`.

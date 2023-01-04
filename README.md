[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/FeedMe-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/FeedMe-rust)


# `FeedMe`
FeedMe is a collection of libraries and tools to create podcast feeds from various sources.


## Example
```sh
# Download the playlist and metadata as mp4
yt-dlp --write-info-json --write-playlist-metafiles \
    --write-thumbnail --convert-thumbnails=jpg \
    --restrict-filenames \
    --format="bestvideo[ext=mp4][vcodec^=avc1]+bestaudio[ext=m4a] \
        /best[ext=mp4][vcodec^=avc1]/best[ext=mp4]/best" \
    https://youtu.be/my_playlist

# Extract and canonicalize the yt-dlp generated metadata
feedme-ytdlp

# Export the webroot and server URL
#   This is necessary to build an absolute URL from a filesystem
#   path that leads to your server
export FEEDME_WEBROOT=/var/www
export FEEDME_BASE_URL=https://example.org

# Generate the feed into feed.rss
feedme-feed
```

# Instadig

Automatically sync your liked articles on [Instapaper](https://www.instapaper.com) to [Linkding](https://github.com/sissbruecker/linkding).

Sync runs every 10 minutes.

## Setup

### Configuration

Before running the application you must define the following environment variables:

* `INSTAPAPER_FEED_URL` – RSS feed of your liked/starred articles which you can find in the source of `https://www.instapaper.com/liked` page.
* `LINKDING_API_PATH` – Full URL with `/api` endpoint of your Linkding instance (don't include the trailing slash).
* `LINKDING_TOKEN` – Your authentication token for Linkding.

### Docker Compose

```yaml
version: "3.8"
services:

  instadig:
    image: ghcr.io/gentlecat/instadig:latest
    container_name: instadig
    restart: unless-stopped
    environment:
      INSTAPAPER_FEED_URL: https://www.instapaper.com/starred/rss/XXXXXXXX/XXXXXXXX
      LINKDING_API_PATH: https://your.linkding.instance/api
      LINKDING_TOKEN: YOUR_TOKEN
```

### Development

```shell
cp .env.example .env
```

Then update the *.env* file with the right configuration.

# OpenWhoop

OpenWhoop is project that allows you to download heart rate data directly from your Whoop4.0 device without Whoop subscription or Whoops servers, making data your own.

### How to Run?

First you need to copy `.env.example` into `.env` and then scan until you find your Whoop device:
```sh
cp .env.example .env
cargo run -r -- scan
```

### How to Run the webapp using docker?

You can built the webapp manually using docker. Execute the following commands in the root folder `openwhoop`.

```sh
docker compose build
docker compose up
```

The app runs now at `http://localhost:8080/`.

Or you can get the prebuilt image from docker hub:
TODO


## TODO:

- [ ] Sleep detection, for most of things like strain, recovery, HRV, etc..., I have been able to reverse engineer calculations, but I need reverse engineer sleep detection and activity detection before they can be automatically calculated
- [ ] Mobile/Desktop app
- [ ] Sp02 readings
- [ ] Temperature readings
# tagzen

An api microservice to tag and catagorise tv & movies at ease

## About

Here's the help infomation taken from the index route (`/`) of this api:

```none
ROUTE /


About
    Microservice api for tagging television shows and movies for use publically
    for free, forever. Created by https://ogriffiths.com. Help is available for
    each route and endpoint on GET access. Running on v{} currently with the
    foss repository contained inside of https://github.com/owez/tagzen/.


Child routes/endpoints
    - /tv: Television show tagging, allowing single episode or seasonal tagging
    - /music: Music tagging for single songs or albums
```

## Running

Clone then build with `cargo build --release`. The outputted binary from this will be outputted inside of `target/release` which can be optionally stripped and ran as a static binary. All help infomation is contained inside of the api itself, so just visit the index page once it's up and running 👐

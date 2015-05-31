# heroku-tiny-http-hello-world

"Hello World" using [tiny-http](https://github.com/frewsxcv/tiny-http) on Heroku

## How to use

```sh
# Clone this repository
git clone git@github.com:frewsxcv/heroku-tiny-http-hello-world.git

# Add the remote git on Heroku
heroku git:remote -a "<your app name here>"

# Use a Rust buildpack
heroku buildpacks:set https://github.com/emk/heroku-buildpack-rust

# Deploy
git push heroku master

# Scale up web process
heroku ps:scale web=1
```

## License

All code in this repository is licensed as [CC0](https://creativecommons.org/publicdomain/zero/1.0/)

# Strife + Actix POC
This is an example project that uses [Strife](https://strife.app/) on [Actix](https://actix.rs/).

## Technologies and frameworks
* [Actix](https://actix.rs/) as the backing server
* [Maud](https://maud.lambda.xyz/) as the HTML template engine
* [Alpine.js](https://alpinejs.dev/) JS library for reactivity/real-time preview (_Only included when compiled with the `realtime` feature_)
* [TailwindCSS 4](https://tailwindcss.com/) for CSS (_Note: Uses the [Standalone Tailwind CLI](https://github.com/tailwindlabs/tailwindcss/releases/latest), no Node/NPM in this project_)

## Requirements
* [Rust](https://www.rust-lang.org/tools/install)
* [TailwindCSS 4 CLI](https://github.com/tailwindlabs/tailwindcss/releases/latest) available on `$PATH` as binary `tailwindcss`
* Some SSL dependencies for secure RavenDB communications on Linux:
```
sudo apt-get install libssl-dev pkg-config
```
* For localhost HTTPS, and Strife port redirect, use your reverse proxy of choice. I like [Caddy](https://caddyserver.com/):
```
caddy reverse-proxy --from localhost:4322 --to localhost:8080
```

## Setup and run
### Prepare `.env` file
Prepare an `.env` file in the folder root. It must contain the following:
* `STRIFE_CERTIFICATE` - Base64 encoded pkcs12 RavenDB client certificate string
* `STRIFE_CERTIFICATE_PASSWORD` - Certificate password
* `STRIFE_DATABASE_URLS` - Comma separated string of Strife Database endpoints (only the first one is used here)
* `STRIFE_DATABASE` - Name of the Strife database

### Verify TailwindCSS 4 CLI
This project uses the standalone [TailwindCSS 4 CLI](https://github.com/tailwindlabs/tailwindcss/releases/latest) to search through source code and generate a CSS. This is performed by the Rust build script (`build.rs`) from `input.css`.

Make sure that the TailwindCSS CLI is available with `tailwindcss` or modify the command in `build.rs` to use whatever method you want to generate the CSS.

### Build or run
#### Without JS/realtime preview
`cargo run` will build and start a debug configured Actix web server on `http://localhost:8080`. With a properly configured `.env` you should be able to see your site.

#### With JS/realtime preview
`cargo run --features realtime` will also build and start a debug configured Actix web server on `http://localhost:8080`, but this configuration will load in Alpine.js and Strife JS SDK and hook up reactivity for use in Strife Studio. Rendering is now handled through JavaScript and Alpine.js instead.

_Note: For Strife Studio and `localhost` you will need HTTPS, you can use e.g. [Caddy](https://caddyserver.com/) for this. To redirect from port `4322` to Actix `8080`, and apply a trusted localhost certificate in Caddy, use:_

```
caddy reverse-proxy --from localhost:4322 --to localhost:8080
```
#### Static content
JS and CSS is placed in the `public` folder, which is copied to the build directory when compiling. If you want to compile this to a single binary and run it somewhere, remember to include the `public` folder along with the binary; the CSS is required and the `stores.js` is required when in `realtime`.

#### Debug raw Strife content
You can use the `/rawjson/` path to view the raw Strife data retrieved by the middleware, for example:

```
http://localhost:8080/rawjson/instant-preview-instant-groove
```
or
```
http://localhost:8080/rawjson/
```

## Project structure and idea
The idea of this POC is to have two different "content flows"; one for Strife Studio/reactive/real time work and one for purely displaying the content as efficiently as possible.

Instead of having an "all in one package", using Rust conditional configurations, `#[cfg(feature = ...)]`, entire code segments and modules can be included or excluded making for a very small and optimized binary when Strife Studio interaction is not required.

The project is built on Actix, using its Extractor feature (a middleware) to inject Strife content on every request. The middleware uses RavenDB's REST API to retrieve the content matching the current URL path with the `Content/ByUrl` index.

### Content flow - without real time
When the site is compiled without the `realtime` feature, the REST API middleware is the only connection to Strife. No JavaScript is used at all. The Strife data is injected on each service handler as a typed struct and all of the site content is built on the server.

### With realtime
In `realtime` mode, the middleware data is still injected in the request (for an initial rendering) but Alpine.js will take over after its initialization and connect to the Strife SDK via `subscribe` and handle everything from there. If you want real time preview in Strife Studio, this is the way to go.

## TODOs and misc
* This is  a _POC_
* Code duplication between the two content flows can be handled smarter

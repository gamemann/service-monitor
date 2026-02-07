### ⚠️ WIP & Not Yet Functional
A simple service monitor written in Rust. Each *service* contains a check and optional success and failure **alerts**.

The only type of check supported right now is HTTP/HTTPS which simply checks the status code of a web page (or a timeout).

At this time, the only type of alert type supported is Discord. This alert executes a configured web hook and allows you to send basic or raw data (e.g. being able to post a message inside a channel).

## Building & Running
Building and running is simple once you've installed Rust onto your system.

```bash
# Apt: For Debian/Ubuntu-based systems
# Install Git and curl
sudo apt install -y git curl

# Install Rust:
# Use curl and rustup to install Rust onto your system
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone respository.
git clone https://github.com/gamemann/service-monitor

# Change directory to project.
cd service-monitor

# Build and run for dev.
cargo run

# Build and run for release.
cargo run --release

# Just build.
cargo build

# To pass CLI arguments, separate Cargo and tool arguments with '--'.
cargo run --release -- -c ./my-test-conf.json
```

## CLI Usage
There are a couple of command line arguments supported by this tool.

You may use these arguments by separating the arguments with `cargo run` with `--`. For example:

```bash
cargo run --release -- --list
```

| Args | Default | Description |
| ---- | ------- | ----------- |
| `-c --cfg` | `./settings.json` | The path to the config file to load. |
| `-l --list` | - | If set, loads the config, prints the new values, and exits. |

## Configuration
We use JSON to store configuration settings. By default, the program attempts to load the file `./settings.json` which is not created by default. You may copy the [`settings.ex.json`](./settings.ex.json) file to `settings.json`. If you'd like to change the config file location, please use the command line arguments detailed above.

| Name | Type | Default | Description |
| ---- | ---- | ------- | ----------- |
| debug_lvl | string(`"debug" \| "info" \| "warn" \| "error"`) | `"info"` | The debug level as a string. |
| log_dir | string | `NULL` | The directory to store logs in. |
| services | array(Service Object) | `[...]` | The array of services to setup. |

<details>
    <summary>Example</summary>

```json
{
    "debug_lvl": "debug",
    "log_dir": null,
    "services": [
        {
            "name": "TMC Website",

            "check": {
                "cron": "0 * * * * *",
                "type": "http",
                "http": {
                    "method": "GET",
                    "url": "https://moddingcommunity.com",
                    "timeout": 5
                }
            },
            "alert_pass": {
                "type": "discord",
                "discord": {
                    "webhook_url": "https://discord.com/api/webhooks/xxxxx/yyyyy",
                    "timeout": 5,
                    "content_basic": "TMC website is back online!",
                    "content_raw": null
                }
            },
            "alert_fail": {
                "type": "discord",
                "discord": {
                    "webhook_url": "https://discord.com/api/webhooks/xxxxx/yyyyy",
                    "timeout": 5,
                    "content_basic": "TMC website is offline!",
                    "content_raw": null
                }
            }
        }
    ]
}
```
</details>

### Service Object
This object contains settings on a service.

| Name | Type | Default | Description |
| ---- | ---- | ------- | ----------- |
| name | string | `NULL` | The name of the service. |
| fails_cnt_to_alert | u32 | `3` | The number of fails in a row before the failed alert is trigger. |
| lat_max_track | u32 | `10` | How many latency values to store inside of the array (used when calculating latency stats). |
| check | Check Object | `{...}` | Settings related to the service's check. |
| alert_pass | Alert Object | `{...}` | Settings related to the service's alert pass configuration. |
| alert_fail | Alert Object | `{...}` | Settings related to the service's alert fail configuration. |

### Check Object
This object contains settings for a service's check.

| Name | Type | Default | Description |
| ---- | ---- | ------- | ----------- |
| cron | string | `"0 * * * * *"` | The check scheduler's cron string. Read [here](https://crates.io/crates/tokio-cron-scheduler) for more info. |
| check_type | string(`"http"`) | `"http"` | The check type. |
| http | HTTP Object | `{...}` | The HTTP check object. |

#### HTTP Object
This object contains settings for a HTTP/HTTPS check.

| Name | Type | Default | Description |
| ---- | ---- | ------- | ----------- |
| timeout | u32 | `10` | The request timeout before failing. |
| method | string | string(`"get" \| "post" \| "put" \| "delete" \| "patch"`) | `"get"` | The HTTP method to use when sending the request. |
| url | string | `"http://127.0.0.1"` | The URL to send the HTTP request to. |
| headers | string => string mapping | `{"...": "..."}` | An optional object of headers (string => string). |

### Alert Object
This object contains settings for a service's alert.

| Name | Type | Default | Description |
| ---- | ---- | ------- | ----------- 
| type | string(`"discord"`) | `"discord"` | The type of alert. |
| discord | Discord Object | `{...}` | The Discord alert object. |

#### Discord Object
This object contains settings for the Discord alert type which allows the execution of a Discord webhook.

| Name | Type | Default | Description |
| ---- | ---- | ------- | ----------- 
| webhook_url | string | `NULL` | The Discord webhook URL. |
| timeout | u32 | `10` | The Discord webhook request's timeout in seconds. |
| content_basic | string | `NULL` | If `content_raw` isn't set, the value of this is passed to the `content` string inside of the body of the Discord webhook request. |
| content_raw | string | `NULL` | If set, passes the value of this as the body when sending the Discord webhook request. |

## My Motives
I tried learning Rust a couple of years ago, but unfortunately never stuck with it. However, since I will most likely be using Rust in the future for my job, I need/want to relearn it. I figured a good starting point is to create this service monitor that I will be using for my [modding project](https://moddingcommunity.com)!

I'm pretty much *winging* Rust on this project. I haven't read much documentation on Rust and wanted to see how far I can go by Googling, using AI (to explain code and why we use x, y, and z).

I'm sure there are many ways to improve the code I've written in this project and if you see any suggestions, please feel free to make a PR!

## Credits
* [Christian Deacon](https://github.com/gamemann)
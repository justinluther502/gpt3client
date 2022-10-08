# WritingHelper

Generate GPT-3 text completions quickly using an executable wrapper for the OpenAI GPT-3 Completions API, two text files, and a config file.

## Install

### Binaries

None included yet.

#### Building from source

Run

```shell
git clone https://github.com/justinluther502/WritingHelper.git
cd WritingHelper
cargo build --release
```

to build the application, then drop the following 4 files anywhere in the same directory:

- prompt.txt
- suffix.txt
- api_config.toml
- 

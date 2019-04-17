# hackscanner

> A filesystem scanner that will search for files with suspicious content or file paths.

## Usage

```bash
hackscanner /root/directory/to/scan/
```

### What does this do?

The scanner will go through each file in the given root directory (or the current working directory if none is given). 
Each file will be checked against all the defined rules. The severity values of all matching rules will be summed up to 
build a rating for the checked file.

Finally the results will be sorted by rating and be printed to the screen. 


## Rules

The scanner has a set of [builtin rules](src/rule/builtin.rs), but can easily be extended with custom rules. 
Custom rules can be defined in a configuration file which is applied with the `-c, --configuration <configuration>` option.

```bash
hackscanner /root/directory/to/scan/ -c /configuration.yaml
# or
hackscanner /root/directory/to/scan/ -c /configuration.json
```

The configuration file can be a JSON or YAML file.

### JSON configuration file

```json
[
    {
        "name": "some rule",
        "path": "some/path",
        "content": "some bad content",
        "severity": "CRITICAL"
    },
    {
        "name": "some whitelist rule",
        "path": "\\.php",
        "content": "love",
        "severity": "WHITELIST"
    }
]
```

### YAML configuration file

```yaml
- name: some rule
  path: some/path
  content: some bad content
  severity: CRITICAL

- name: some whitelist rule
  path: "\\.php"
  content: love
  severity: WHITELIST
```


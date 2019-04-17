# hackscanner

> A filesystem scanner that will search for files with suspicious content or file paths.

## Usage

```bash
hackscanner /root/directory/to/scan/
```

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


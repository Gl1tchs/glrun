# glrun

Command line utility for cross platform scripts.

## Example:
```
# script.gs
@windows
- echo "hello windows"
- explorer %USERPROFILE%
@linux
- echo "hello linux"
- ls ~
--
echo 'hellow'
echo 'world'
--
```

```
glrun script.gs
```

## Usage:
```
gl1tch in ~/Projects/glrun λ glrun --help

Cross-Platform script command runner.

USAGE:
    glrun [OPTIONS] <script>

ARGS:
    <script>    Sets the script file or URL to use

OPTIONS:
    -h, --help        Print help information
    -v, --validate    Validate the script only, don't execute it
    -y, --yes         Do not ask for confirmation before running the script
```

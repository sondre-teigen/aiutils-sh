# aiutils-sh

Shell utilities for operating LLM APIs.

Supported API providers:
- OpenAI

## Table of Contents
- [Primitive Utilities](#primitive-utilities)
  - [Prompt Construction and Completion](#prompt-construction-and-completion)
  - [Completion Output Handling](#completion-output-handling)
  - [Embeddings](#embeddings)
- [Extra Utilities](#extra-utilities)

## Primitive Utilities

Primitive utilities implemented in Rust.

The usage and functionality of these utilities are intentionally verbose and low-level, they are intended to be combined into richer Shell scripts or functions.

### Prompt Construction and Completion

#### `aimessage`

Convert stdin into a JSON message.

Example:

```bash
$ echo "Hello, world!" | aimessage
```

Formatted output:

```json
[{ "role": "user", "content": "Hello, world!\n" }]
```

#### `aicat-messages`

Collect JSON messages into a single JSON array.

Example:

```bash
$ echo "Hello, world!" | aimessage | aicat-messages <(echo "Respond only in Spanish" | aimessage --role developer) -
```

Formatted output:

```json
[
  { "role": "developer", "content": "Respond only in Spanish\n" },
  { "role": "user", "content": "Hello, world!\n" }
]
```

#### `aicomplete`

Read JSON messages and generate a completion.

Example:

```bash
$ echo "Hello, world!" | aimessage | aicat-messages <(echo "Respond only in Spanish" | aimessage --role developer) - | aicomplete
```

Output:

```plaintext
¡Hola, mundo!
```

#### `aifile`

Format a file as a Markdown code block.

Example:

```bash
$ aifile LICENSE --head 2 --tail 2
```

Output (triple backticks mangled with a preceding apostrophe):

````plaintext

File: `LICENSE`
'```
MIT License
[... 17 lines omitted]
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
'```
````

#### `aiexec`

Execute a command and capture the output as a Markdown code block.

Example:

```bash
$ aiexec --head 3 -- cat --help
```

Output (triple backticks mangled with a preceding apostrophe):

````plaintext

Command: `cat --help`
'```console
Usage: cat [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

[... 23 lines omitted]
'```
````

#### Combined example

The prompt construction utilities can be combined into complex prompts.

Example:

```bash
$ echo "Why does my build fail?" | cat - <(aiexec rustc <(echo "f main() {}")) | aimessage | aicat-messages <(echo "Respond in 140 characters or less" | aimessage --role developer) - | aicomplete --stream
```

Output:

```plaintext
Your build fails because you used `f` instead of `fn`. Change `f main()` to `fn main()`.
```

### Completion Output Handling

- **`aisponge`**  
  Collect stdin into a file line by line and then truncating. As opposed to moreutils `sponge` which truncates first.

  The purpose of `aisponge` is to be functionally equivalent to moreutils `sponge`, while create a slightly more appealing visual when interactively overwriting files.

- **`aiextract-block`**  
  Extract Mardkown code blocks from stdin.

  Example:

  ```bash
  $ echo "Write a Python function which prints out Hello, world!" | aimessage | aicomplete | aiextract-block --redirect-rest "noise.txt"
  ```

  Output:

  ```python
  def print_hello_world():
      print("Hello, world!")

  # Call the function to see the output
  print_hello_world()
  ```

  `noise.txt` output:

  ```plaintext
  Certainly! Here is a simple Python function that prints "Hello, world!":

  You can run this code in any Python environment to see the output.
  ```

### Embeddings

#### `aiembed`

Generate embeddings from files.

Example:
```bash
$ aiembed LICENSE Cargo.toml
```

Output (truncated Base64):
```json
{
  "LICENSE": "qJGHPEL...5oA6",
  "Cargo.toml": "bHMJvXcXF...eo08"
}
```

#### `aiembed-score`

Compute similarities from `aiembed` embeddings.

Example
```bash
$ aiembed printf:<(man printf) fwrite:<(man fwrite) fputs:<(man fputs) > docs.json
$ echo "What C function should I use for writing a string to a stream?" | aiembed - | aiembed-score - docs.json --score
```

Output:
```plaintext
0.4462 fputs
0.3498 fwrite
0.2820 printf
```

#### `aicat-embeddings`

Combine JSON outputs from `aiembed` into a single file

Example:
```bash
$ aiembed LICENSE > embed-license.json
$ aiembed Cargo.toml > embed-cargo-toml.json
$ aicat-embeddings embed-license.json embed-cargo-toml.json
```

Output (truncated Base64):
```json
{
  "Cargo.toml": "bHMJvXcXFb...eo08",
  "LICENSE": "qJGHPELi...5oA6"
}
```

## Extra Utilities

Sample Bash utilities made from the primitive utilities.

The purpose of these utilities is to showcase richer funcionality is implemented in pure Bash using the primitive utilities.

### `aiedit`

Edit a file on the filesystem via an interactive chat.

### `aichat`

Interactive chat completions.

### `aicmd`

Generate a terminal command based on a descriptive attempt.

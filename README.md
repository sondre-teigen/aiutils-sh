# aiutils-sh
Shell utilities for operating OpenAI API

## Utilities

### Prompt Construction
- **`aimessage`**  
  Convert text into structured API markup messages, specifying the role for context.

- **`aicat-messages`**  
  Collect and splice together multiple API markup messages from specified files.

- **`aifile`**  
  Format an input file with markup annotations.

- **`aiexec`**  
  Execute a command in the shell and format the captured output with markup annotations.

### Completion
- **`aicomplete`**  
  Generate text completions based on API markup messages in the specified input file.

### Output Handling
- **`aisponge`**  
  Collect stdin input into a file, preserving each line without truncation.

- **`aiextract-block`**  
  Parse and extract code blocks from markdown files.

### Embeddings
- **`aiembed`**  
  Generate embeddings from the contents of specified input files.

- **`aiembed-score`**  
  Compare and rank embedding similarities to identify relevant results against a query.

- **`aicat-embeddings`**  
  Manage and organize API message embeddings from various files.

## Extra Utilities

### `aiprompt`
Generate an AI completion based on a prompt from stdin.

### `aiedit`
Interactively edit a file with an AI model and chat.

### `aiask`
Ask an AI model a standalone question from stdin.

### `aichat`
Chat interactively with an AI model.

### `aicmd`
Generate a terminal command with an AI model.

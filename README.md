# rlu: Rust Logseq Utility

`rlu` is a Rust-based command-line utility designed to interact with Logseq, a privacy-first, open-source knowledge base that works on top of local Markdown and Org-mode files. This utility provides various functionalities including adding journal notes, displaying entries, and managing content in Logseq.

## Features

- **Add Journal Entries**: Add new entries to your Logseq journal from stdin or directly from the command line.
- **Show Journal Entries**: Display the entries for a specific date.
- **Get Journal Entry**: Retrieve a specific entry by its ID.
- **Output Entry Content**: Output the full content of a specific entry.
- **Add Content**: Add content to the start or end of an existing entry.
- **Add Child Node**: Add a child node to an existing entry.
- **Delete Entry**: Delete a specific entry by its ID.

## Installation

To install `rlu`, you'll need to have [Rust](https://www.rust-lang.org/) installed. Once you have Rust installed, you can build the project using Cargo:

```sh
git clone https://github.com/yourusername/rlu.git
cd rlu
cargo build --release
```

The compiled binary will be located in `target/release/`.

## Usage

To use the `rlu` command-line tool, you can run the following commands:

### Commands

- **Add a Journal Note**:
  ```sh
  rlu add --content "Your journal content" --date "2023-10-05"
  ```

  You can also pipe content from stdin:
  ```sh
  echo "Your journal content" | rlu add
  ```

- **Show Journal Entries**:
  ```sh
  rlu show --date "2023-10-05"
  ```

- **Get Journal Entry**:
  ```sh
  rlu get --entry_id "entry-uuid"
  ```

- **Output Entry Content**:
  ```sh
  rlu output-content --entry_id "entry-uuid"
  ```

- **Add Content to Start**:
  ```sh
  rlu add-to-start --entry_id "entry-uuid" --content "New start content"
  ```

- **Append Content to End**:
  ```sh
  rlu append-to-end --entry_id "entry-uuid" --content "New end content"
  ```

- **Add Child Node**:
  ```sh
  rlu add-child-node --entry_id "parent-id" --content "Child node content"
  ```

- **Delete Entry**:
  ```sh
  rlu delete --entry_id "entry-uuid"
  ```

### Environment Variables

Ensure that the following environment variables are set:

- `LOGSEQ_API_URL`: The URL for the Logseq API (default: `http://127.0.0.1:12315/api`).
- `LOGSEQ_API_KEY`: Your Logseq API key for authorization.

### Example

```sh
export LOGSEQ_API_URL="http://your-logseq-url/api"
export LOGSEQ_API_KEY="your-logseq-api-key"

rlu add --content "Meeting notes for today" --date "2023-10-05"
rlu show --date "2023-10-05"
```

## Dependencies

- `reqwest`: For making HTTP requests.
- `serde` and `serde_json`: For serializing and deserializing JSON.
- `chrono`: For handling date and time.
- `clap`: For command-line argument parsing.
- `log` and `env_logger`: For logging.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

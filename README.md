# rlu: Rust Logseq Utility
![](https://njf.gitbook.io/~gitbook/image?url=https%3A%2F%2F1687842678-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252F1FLuwCkKYFtZVgzhV4kt%252Fuploads%252FEnAZSuk0kkVhiToTCojf%252FDALL%25C2%25B7E%25202024-05-22%252011.22.25%2520-%2520Create%2520a%2520banner%2520image%2520for%2520a%2520GitBook%2520titled%2520%27RLU_%2520Rust%2520Logseq%2520Utility%27.%2520The%2520design%2520should%2520be%2520modern%2520and%2520clean%252C%2520featuring%2520the%2520Rust%2520programming%2520language%2520.webp%3Falt%3Dmedia%26token%3Dad1d446e-2690-4cf3-b232-b5a865131ad9&width=1248&dpr=1&quality=100&sign=9d2f576f94e83e0ef4558c37bd0818c55b94197bd58ed5f39a97c56dc0b49ee3)

[![GitBook](https://img.shields.io/badge/GitBook-Read%20Docs-blue)](https://njf.gitbook.io/rust-logseq-utility-rlu/)



`rlu` is a Rust-based command-line utility designed to interact with Logseq, a privacy-first, open-source knowledge base that works on top of local Markdown and Org-mode files. This utility provides various functionalities including adding journal notes, displaying entries, and managing content in Logseq.

[![Demo](https://cdn.loom.com/sessions/thumbnails/e10dbf4489d8475da92c3618e1403294-1716346175718-with-play.gif)](https://www.loom.com/share/e10dbf4489d8475da92c3618e1403294?sid=e7f59812-c9ab-4e3e-94c6-460a378e9ce2)


## Features

- **Add Journal Entries**: Add new entries to your Logseq journal from stdin or directly from the command line.
- **Show Journal Entries**: Display the entries for a specific date.
- **Get Journal Entry**: Retrieve a specific entry by its ID.
- **Output Entry Content**: Output the full content of a specific entry.
- **Add Content**: Add content to the start or end of an existing entry.
- **Add Child Node**: Add a child node to an existing entry.
- **Delete Entry**: Delete a specific entry by its ID.

## Installation

To install `rlu`, you'll need to have [Rust](https://www.rust-lang.org/) installed (builds coming soon). Once you have Rust installed, you can build the project using Cargo:

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

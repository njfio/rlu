const BASEPATH = "/Users/n/.rlu/";

const completionSpec = {
    name: "rlu",
    description: "CLI for interacting with Logseq",
    subcommands: [
        {
            name: "output-content",
            description: "Output the content of a specific journal entry",
            options: [
                {
                    name: "--entry-id",
                    description: "The ID of the journal entry to get",
                    args: {
                        name: "entry_id",
                        description: "The ID of the journal entry",
                        generators: {
                            script: (context) => {
                                const dateIndex = context.indexOf("--date");
                                const date = dateIndex !== -1 && context[dateIndex + 1] ? context[dateIndex + 1] : new Date().toISOString().split("T")[0];
                                return ["bash", "-c", `${BASEPATH}fetch_entry_ids.sh ${date}`];
                            },
                            postProcess: (out) => {
                                if (out.startsWith("fatal:")) {
                                    return [];
                                }
                                return out.split("\n").map((entry) => {
                                    const [id, ...desc] = entry.split(" ");
                                    return {
                                        name: desc.join(" ").trim() || "Entry ID",
                                        description: id,
                                        insertValue: id.trim(), // Only the ID
                                    };
                                });
                            },
                        },
                    },
                },
                {
                    name: "--date",
                    description: "The date to filter journal entries",
                    args: {
                        name: "date",
                        description: "The date to filter journal entries by (YYYY-MM-DD)",
                        default: new Date().toISOString().split("T")[0],
                    },
                },
            ],
        },
        {
            name: "show",
            description: "Show journal entries for a specific date",
            options: [
                {
                    name: "--date",
                    description: "The date of the journal entries to show",
                    args: {
                        name: "date",
                        description: "The date to filter journal entries by (YYYY-MM-DD)",
                        default: new Date().toISOString().split("T")[0],
                    },
                },
            ],
        },
        {
            name: "add",
            description: "Add a new journal entry",
            options: [
                {
                    name: "--title",
                    description: "Title of the journal entry",
                    args: {
                        name: "title",
                        description: "Title of the entry",
                    },
                },
                {
                    name: "--content",
                    description: "Content of the journal entry",
                    args: {
                        name: "content",
                        description: "Content of the entry",
                    },
                },
                {
                    name: "--date",
                    description: "Date for the journal entry (YYYY-MM-DD)",
                    args: {
                        name: "date",
                        description: "The date of the journal entry",
                        default: new Date().toISOString().split("T")[0],
                    },
                },
            ],
        },
        {
            name: "add-to-start",
            description: "Add content to the start of a journal entry",
            options: [
                {
                    name: "--entry-id",
                    description: "The ID of the journal entry to update",
                    args: {
                        name: "entry_id",
                        description: "The ID of the journal entry",
                        generators: {
                            script: ["bash", "-c", `${BASEPATH}list_entry_ids.sh`],
                            postProcess: (out) => {
                                if (out.startsWith("fatal:")) {
                                    return [];
                                }
                                return out.split("\n").map((id) => ({
                                    name: id.trim(),
                                    description: "Entry ID",
                                    insertValue: id.trim(),
                                }));
                            },
                        },
                    },
                },
                {
                    name: "--content",
                    description: "Content to add at the start",
                    args: {
                        name: "content",
                        description: "Content to add",
                    },
                },
            ],
        },
        {
            name: "append-to-end",
            description: "Append content to the end of a journal entry",
            options: [
                {
                    name: "--entry-id",
                    description: "The ID of the journal entry to update",
                    args: {
                        name: "entry_id",
                        description: "The ID of the journal entry",
                        generators: {
                            script: ["bash", "-c", `${BASEPATH}list_entry_ids.sh`],
                            postProcess: (out) => {
                                if (out.startsWith("fatal:")) {
                                    return [];
                                }
                                return out.split("\n").map((id) => ({
                                    name: id.trim(),
                                    description: "Entry ID",
                                    insertValue: id.trim(),
                                }));
                            },
                        },
                    },
                },
                {
                    name: "--content",
                    description: "Content to append",
                    args: {
                        name: "content",
                        description: "Content to append",
                    },
                },
            ],
        },
        {
            name: "add-child-node",
            description: "Add a child node to a journal entry",
            options: [
                {
                    name: "--entry-id",
                    description: "The ID of the journal entry to update",
                    args: {
                        name: "entry_id",
                        description: "The ID of the journal entry",
                        generators: {
                            script: ["bash", "-c", `${BASEPATH}list_entry_ids.sh`],
                            postProcess: (out) => {
                                if (out.startsWith("fatal:")) {
                                    return [];
                                }
                                return out.split("\n").map((id) => ({
                                    name: id.trim(),
                                    description: "Entry ID",
                                    insertValue: id.trim(),
                                }));
                            },
                        },
                    },
                },
                {
                    name: "--content",
                    description: "Content of the child node",
                    args: {
                        name: "content",
                        description: "Content of the child node",
                    },
                },
            ],
        },
        {
            name: "delete",
            description: "Delete a journal entry by ID",
            options: [
                {
                    name: "--entry-id",
                    description: "The ID of the journal entry to delete",
                    args: {
                        name: "entry_id",
                        description: "The ID of the journal entry",
                        generators: {
                            script: ["bash", "-c", `${BASEPATH}list_entry_ids.sh`],
                            postProcess: (out) => {
                                if (out.startsWith("fatal:")) {
                                    return [];
                                }
                                return out.split("\n").map((id) => ({
                                    name: id.trim(),
                                    description: "Entry ID",
                                    insertValue: id.trim(),
                                }));
                            },
                        },
                    },
                },
            ],
        },
    ],
    options: [
        {
            name: "--help",
            description: "Print this message or the help of the given subcommand(s)",
        },
        {
            name: "--version",
            description: "Show version of rlu",
        },
    ],
};

export default completionSpec;

use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "docs",
        categories: vec![
            Category {
                name: "Introducing Bionic".to_string(),
                pages: vec![Page {
                    date: "",
                    title: "Introduction",
                    description: "Introducing Bionic",
                    folder: "docs/",
                    markdown: include_str!("../content/docs/index.md"),
                    image: None,
                    author_image: None,
                    author: None,
                }],
            },
            Category {
                name: "Learn Bionic".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "The Console",
                        description: "The Console",
                        folder: "docs/guides/console/",
                        markdown: include_str!("../content/docs/guides/console/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Datasets",
                        description: "Datasets",
                        folder: "docs/guides/datasets/",
                        markdown: include_str!("../content/docs/guides/datasets/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "AI Assistants (RAG)",
                        description: "AI Assistants (RAG",
                        folder: "docs/guides/aiassistants/",
                        markdown: include_str!("../content/docs/guides/aiassistants/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Teams",
                        description: "Teams",
                        folder: "docs/guides/teams/",
                        markdown: include_str!("../content/docs/guides/teams/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Advanced".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Automating Document Upload",
                        description: "Automating Document Upload",
                        folder: "docs/guides/uploading-documents/",
                        markdown: include_str!(
                            "../content/docs/guides/uploading-documents/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Using the API",
                        description: "Using the API",
                        folder: "docs/guides/api/",
                        markdown: include_str!("../content/docs/guides/api/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Admin".to_string(),
                pages: vec![Page {
                    date: "",
                    title: "Managing Models",
                    description: "Managing Models",
                    folder: "docs/guides/managing-models/",
                    markdown: include_str!("../content/docs/guides/managing-models/index.md"),
                    image: None,
                    author_image: None,
                    author: None,
                }],
            },
            Category {
                name: "How-To".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Deploying Jupyter Notebook",
                        description: "Deploying Jupyter Notebook",
                        folder: "docs/guides/jupyter/",
                        markdown: include_str!("../content/docs/guides/jupyter/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Visualising RAG",
                        description: "Visualising RAG",
                        folder: "docs/guides/visual-rag/",
                        markdown: include_str!("../content/docs/guides/visual-rag/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Coding CoPilot",
                        description: "Coding CoPilot",
                        folder: "docs/guides/code-copilot/",
                        markdown: include_str!("../content/docs/guides/code-copilot/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Credit Card Categorising Assistant",
                        description: "Credit Card Categorising Assistan",
                        folder: "docs/guides/howtocategorise/",
                        markdown: include_str!("../content/docs/guides/howtocategorise/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Reference".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Glossary",
                        description: "Glossary",
                        folder: "docs/guides/glossary/",
                        markdown: include_str!("../content/docs/guides/glossary/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Open Source Models",
                        description: "Open Source Models",
                        folder: "docs/guides/opensourcemodels/",
                        markdown: include_str!("../content/docs/guides/opensourcemodels/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Local Bionic".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Try it on a Laptop",
                        description: "Try it on a Laptop",
                        folder: "docs/running-locally/docker-compose/",
                        markdown: include_str!(
                            "../content/docs/running-locally/docker-compose/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Connecting to Ollama",
                        description: "Connecting to Ollam",
                        folder: "docs/running-locally/ollama/",
                        markdown: include_str!("../content/docs/running-locally/ollama/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Deploying To Your Infrastructure".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Quick Install (Linux)",
                        description: "Quick Install (Linux)",
                        folder: "docs/on-premise/install-linux/",
                        markdown: include_str!("../content/docs/on-premise/install-linux/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Quick Install RKE2",
                        description: "Quick Install RKE2",
                        folder: "docs/on-premise/install-rke2/",
                        markdown: include_str!("../content/docs/on-premise/install-rke2/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Install AWS",
                        description: "Install AWS",
                        folder: "docs/on-premise/aws/",
                        markdown: include_str!("../content/docs/on-premise/aws/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Install Google Cloud",
                        description: "Install Google Cloud",
                        folder: "docs/on-premise/gcloud/",
                        markdown: include_str!("../content/docs/on-premise/gcloud/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Connecting Data Sources",
                        description: "Connecting Data Sources",
                        folder: "docs/on-premise/airbyte/",
                        markdown: include_str!("../content/docs/on-premise/airbyte/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Configure Email",
                        description: "Configure Email",
                        folder: "docs/on-premise/email/",
                        markdown: include_str!("../content/docs/on-premise/email/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Upgrading Bionic",
                        description: "Upgrading Bionic",
                        folder: "docs/on-premise/upgrades/",
                        markdown: include_str!("../content/docs/on-premise/upgrades/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Single Sign On",
                        description: "Single Sign O",
                        folder: "docs/on-premise/sso/",
                        markdown: include_str!("../content/docs/on-premise/sso/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Role Based Access Control",
                        description: "Role Based Access Control",
                        folder: "docs/on-premise/rbac/",
                        markdown: include_str!("../content/docs/on-premise/rbac/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Connecting pgAdmin",
                        description: "Connecting pgAdmin",
                        folder: "docs/on-premise/pgadmin/",
                        markdown: include_str!("../content/docs/on-premise/pgadmin/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Cloudflare as Ingress",
                        description: "Cloudflare as Ingress",
                        folder: "docs/on-premise/cloudflare/",
                        markdown: include_str!("../content/docs/on-premise/cloudflare/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Licencing Bionic",
                        description: "Licencing Bioni",
                        folder: "docs/on-premise/licencing/",
                        markdown: include_str!("../content/docs/on-premise/licencing/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
        ],
    }
}

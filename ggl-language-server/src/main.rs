use tower_lsp::lsp_types::*;
use tower_lsp::{LanguageServer, LspService, Server};
use tokio::io::{stdin, stdout};
use dashmap::DashMap;
use std::sync::Arc;
use pest::Parser; // Added this line

// --- Pest Parser Setup ---
extern crate pest;
#[macro_use]
extern crate pest_derive;

// The path is relative to the `main.rs` or `lib.rs` file where this module is defined.
// Assuming ggl-language-server/src/main.rs and src/ggl.pest are siblings in the repo structure.
#[derive(Parser)]
#[grammar = "../../src/ggl.pest"] // Adjust path if your crate structure is different
pub struct GglParser;
// --- End Pest Parser Setup ---


#[derive(Debug)]
struct Backend {
    client: tower_lsp::Client,
    document_map: Arc<DashMap<Url, String>>, // Stores document content
}

impl Backend {
    fn new(client: tower_lsp::Client) -> Self {
        Backend {
            client,
            document_map: Arc::new(DashMap::new()),
        }
    }

    async fn on_change(&self, uri: Url, text: String, version: Option<i32>) {
        self.document_map.insert(uri.clone(), text.clone());
        self.client
            .log_message(MessageType::INFO, format!("File {} updated.", uri))
            .await;

        let diagnostics = match GglParser::parse(Rule::program, &text) {
            Ok(_) => {
                // Successfully parsed, log and return no diagnostics
                // Note: logging is now done after this match block to keep non-Send types local
                vec![]
            }
            Err(e) => {
                // Error parsing, log and convert Pest error to LSP Diagnostic
                // Note: logging is now done after this match block
                // TODO: Implement proper Pest error to LSP Diagnostic conversion
                let pest_error_message = e.to_string(); // Convert to String immediately

                // Placeholder range for now. Proper conversion needed.
                // This needs to parse e.line_col
                let (start_pos, end_pos) = match e.line_col {
                    pest::error::LineColLocation::Pos((line, col)) => {
                        let lsp_line = (line.saturating_sub(1)) as u32;
                        let lsp_char = (col.saturating_sub(1)) as u32;
                        (
                            Position::new(lsp_line, lsp_char),
                            Position::new(lsp_line, lsp_char + 1)
                        )
                    }
                    pest::error::LineColLocation::Span((start_line, start_col), (end_line, end_col)) => {
                        (
                            Position::new((start_line.saturating_sub(1)) as u32, (start_col.saturating_sub(1)) as u32),
                            Position::new((end_line.saturating_sub(1)) as u32, (end_col.saturating_sub(1)) as u32),
                        )
                    }
                };

                let diagnostic = Diagnostic {
                    range: Range::new(start_pos, end_pos),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    source: Some("ggl-parser".to_string()),
                    message: pest_error_message, // Use the converted String
                    ..Default::default()
                };
                vec![diagnostic]
            }
        };

        // Now that all non-Send types from Pest are out of scope (or converted), we can .await
        // Log success or failure based on the diagnostics generated *before* the .await for publishing
        if diagnostics.is_empty() {
            // No need to re-parse here for logging if we already know it's a success
            // from the first parse attempt.
            self.client.log_message(MessageType::INFO, format!("Successfully parsed {}", uri)).await;
        } else {
            self.client.log_message(MessageType::ERROR, format!("Error parsing {}: found {} diagnostics.", uri, diagnostics.len())).await;
        }

        self.client.publish_diagnostics(uri, diagnostics, version).await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        self.client
            .log_message(MessageType::INFO, "GGL Language Server initializing...")
            .await;
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "ggl-language-server".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL, // Send full document content on change
                )),
                position_encoding: Some(PositionEncodingKind::UTF8), // Added this line
                // Add other capabilities like completion, hover, etc. here later
                completion_provider: None, // TODO: Add later
                hover_provider: None,      // TODO: Add later
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "GGL Language Server initialized.")
            .await;
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        self.client
            .log_message(MessageType::INFO, "GGL Language Server shutting down...")
            .await;
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let version = Some(params.text_document.version);
        self.client
            .log_message(MessageType::INFO, format!("File opened: {}", uri))
            .await;
        self.document_map.insert(uri.clone(), text.clone());
        self.on_change(uri, text, version).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        // `content_changes` contains an array of changes. Since we requested
        // `TextDocumentSyncKind::FULL`, the array will contain a single element
        // with the full text of the document.
        let text = params.content_changes.into_iter().next().unwrap().text;
        let version = Some(params.text_document.version);
        self.client
            .log_message(MessageType::INFO, format!("File changed: {}", uri))
            .await;
        self.on_change(uri, text, version).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        // Text might be Some if "includeText" was set to true in DidSaveTextDocumentRegistrationOptions
        // For simplicity, we re-fetch from our map or rely on did_change.
        // If text is None, we can grab it from our document_map.
        // Or, if client settings for didSave include text, we can use params.text
        self.client
            .log_message(MessageType::INFO, format!("File saved: {}", uri))
            .await;
        if let Some(text_content) = self.document_map.get(&uri) {
             // Get version from document_map or handle if not available
            self.on_change(uri.clone(), text_content.clone(), None).await; // version might not be available here
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.client
            .log_message(MessageType::INFO, format!("File closed: {}", uri))
            .await;
        self.document_map.remove(&uri);
    }

    // TODO: Implement other handlers like completion, hover, etc.
}

#[tokio::main]
async fn main() {
    let stdin = stdin();
    let stdout = stdout();

    let (service, socket) = LspService::build(|client| Backend::new(client))
        // .custom_method("custom/notification", Backend::custom_notification) // Example for custom methods
        .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

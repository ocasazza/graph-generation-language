import * as path from 'path';
import { workspace, ExtensionContext, window, ExtensionMode } from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  // The server is assumed to be a separate executable
  // Adjust the path according to your build output and development setup
  // For development, you might point to the debug build of your Rust server
  // For production, this might be a path within the extension package after bundling
  // Determine server path: packaged ('server/') vs. local development (sibling target/debug)
  const isDevMode = process.env.VSCODE_GGL_DEV_MODE === 'true' || context.extensionMode === ExtensionMode.Development;
  let serverModulePath: string;

  if (isDevMode) {
    // Path for local development: assumes ggl-language-server is a sibling directory
    // and we're using a debug build.
    const platformSpecificExecutable = process.platform === 'win32' ? 'ggl-language-server.exe' : 'ggl-language-server';
    serverModulePath = path.join('..', 'ggl-language-server', 'target', 'debug', platformSpecificExecutable);
    window.showInformationMessage('GGL Extension: Running in Development Mode. Server path: ' + context.asAbsolutePath(serverModulePath));
  } else {
    // Path for packaged extension: server binary is expected in 'server/' subdirectory
    // The CI/CD pipeline places the appropriate binary here.
    // For now, assumes 'ggl-language-server' for Linux.
    // A more robust solution would involve the CI placing a platform-specific named binary,
    // or the extension having logic to choose based on `process.platform`.
    const platformSpecificExecutable = process.platform === 'win32' ? 'ggl-language-server.exe' : 'ggl-language-server';
    serverModulePath = path.join('server', platformSpecificExecutable);
    window.showInformationMessage('GGL Extension: Running in Production Mode. Server path: ' + context.asAbsolutePath(serverModulePath));
  }
  const serverCommand = context.asAbsolutePath(serverModulePath);

  // If the extension is launched in debug mode then the debug server options are used
  // Otherwise the run options are used
  const serverOptions: ServerOptions = {
    run: { command: serverCommand, transport: TransportKind.stdio },
    debug: {
      command: serverCommand,
      transport: TransportKind.stdio,
      // options: { env: { RUST_LOG: 'debug' } } // Optional: enable debug logging for the server
    }
  };

  // Options to control the language client
  const clientOptions: LanguageClientOptions = {
    // Register the server for GGL documents
    documentSelector: [{ scheme: 'file', language: 'ggl' }],
    synchronize: {
      // Notify the server about file changes to '.gglrc' files contained in the workspace
      // (Example, if you add configuration files for the GGL language server)
      fileEvents: workspace.createFileSystemWatcher('**/.gglrc')
    }
  };

  // Create the language client and start the client.
  client = new LanguageClient(
    'gglLanguageServer',
    'GGL Language Server',
    serverOptions,
    clientOptions
  );

  // Start the client. This will also launch the server
  window.showInformationMessage('GGL Language Client activating.');
  client.start().then(() => {
    window.showInformationMessage('GGL Language Client started successfully!');
  }).catch(error => {
    window.showErrorMessage(`GGL Language Client failed to start: ${error}`);
  });
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  window.showInformationMessage('GGL Language Client deactivating.');
  return client.stop();
}

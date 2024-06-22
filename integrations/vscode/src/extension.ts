import type { TextDocument, ExtensionContext } from 'vscode';
import { workspace, languages } from 'vscode';
import type { LanguageClientOptions, ServerOptions } from 'vscode-languageclient/node';
import { LanguageClient, MessageStrategy } from 'vscode-languageclient/node';

const tosca_extension = 'yaml.tosca';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  const serverOptions: ServerOptions = {
    run: {
      command: 'toto',
      args: ['ls'],
    },
    debug: {
      command: 'toto',
      args: ['ls'],
    },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: tosca_extension }],
  };

  client = new LanguageClient(
    'totosca',
    'TOSCA language server',
    serverOptions,
    clientOptions
  );

  checkAllDocumentsExtensions();

  client.start();

  let disposable = workspace.onDidOpenTextDocument(checkExtension);
  context.subscriptions.push(disposable);

  disposable = workspace.onDidSaveTextDocument(checkExtension);
  context.subscriptions.push(disposable);
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}

function checkAllDocumentsExtensions() {
  for (const textDocument of workspace.textDocuments) {
    checkExtension(textDocument);
  }
}

const tosca_regex = /^tosca_definitions_version: /m
const yaml_regex = /.yaml$/

function checkExtension(textDocument: TextDocument) {
  if (yaml_regex.test(textDocument.fileName) && tosca_regex.test(textDocument.getText())) {
    languages.setTextDocumentLanguage(textDocument, tosca_extension);
  }
}
